use crate::api::media::models::{MediaMetaData, PostMediaMetadata};
use crate::application::models::Pagination;
use crate::domain::media::metadata::{MediaTitle, ValidationError};
use crate::infrastructure::media::repository::{
    create_media_metadata, delete_media_metadata, get_media_metadata, get_media_metadata_by_id,
};
use axum::http::StatusCode;
use axum::{Form, extract::Path, extract::Query, extract::State, response::Json};
use sqlx::PgPool;

pub async fn get_media_by_id(
    State(pool): State<PgPool>,
    id: Query<i32>
) -> Result<(StatusCode, Json<MediaMetaData>), (StatusCode, &'static str)> {

    let row = get_media_metadata_by_id(
        &pool,
        *id
    )
    .await
    .map_err(|err| {
        eprintln!("Failed to get media metadata: {err}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to get media metadata",
        )
    })?;

    let response = MediaMetaData::try_from(row).map_err(|err| {
        eprintln!("Failed to convert media metadata: {err:?}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to build media response",
        )
    })?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_media(
    State(pool): State<PgPool>,
    pagination: Query<Pagination>,
) -> Result<(StatusCode, Json<Vec<MediaMetaData>>), (StatusCode, &'static str)> {
    let row = get_media_metadata(&pool, pagination.page, pagination.step)
        .await
        .map_err(|err| {
            eprintln!("Failed to get media metadata: {err}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to save media metadata",
            )
        })?;

    let response = row
        .into_iter()
        .map(MediaMetaData::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| {
            eprintln!("Failed to convert media metadata: {err:?}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to build media response",
            )
        })?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn post_media(
    State(pool): State<PgPool>,
    Form(params): Form<PostMediaMetadata>,
) -> Result<(StatusCode, Json<MediaMetaData>), (StatusCode, &'static str)> {
    let title = MediaTitle::new(params.title).map_err(|err| match err {
        ValidationError::EmptyTitle => {
            (StatusCode::BAD_REQUEST, "Title must not be empty")
        }
    })?;

    let row = create_media_metadata(
        &pool,
        title.into_inner(),
        params.filename.clone(),
        params.mime_type.clone(),
        params.duration_ms,
        String::from("uploading"),
    )
    .await
    .map_err(|err| {
        eprintln!("Failed to create media metadata: {err}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to save media metadata",
        )
    })?;

    let response = MediaMetaData::try_from(row).map_err(|err| {
        eprintln!("Failed to convert media metadata: {err:?}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to build media response",
        )
    })?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn delete_media(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<MediaMetaData>), (StatusCode, &'static str)> {
    let row = delete_media_metadata(&pool, id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Media not found"),
            other => {
                eprintln!("Failed to delete media metadata: {other}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to delete media metadata",
                )
            }
        })?;

    let response = MediaMetaData::try_from(row).map_err(|err| {
        eprintln!("Failed to convert media metadata: {err:?}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to build media response",
        )
    })?;

    Ok((StatusCode::OK, Json(response)))
}
