
#[get("/new")]
async fn new() -> Template {
    Template::render("new", &Context::default())
}

#[post("/", data = "<post_form>")]
async fn create(conn: Connection<'_, Db>, post_form: Form<post::Model>) -> Flash<Redirect> {
    let db = conn.into_inner();

    let form = post_form.into_inner();

    post::ActiveModel {
        title: Set(form.title.to_owned()),
        text: Set(form.text.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert post");

    Flash::success(Redirect::to("/"), "Post successfully added.")
}

#[post("/<id>", data = "<post_form>")]
async fn update(
    conn: Connection<'_, Db>,
    id: i32,
    post_form: Form<post::Model>,
) -> Flash<Redirect> {
    let db = conn.into_inner();

    let post: post::ActiveModel = Post::find_by_id(id).one(db).await.unwrap().unwrap().into();

    let form = post_form.into_inner();

    db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            post::ActiveModel {
                id: post.id,
                title: Set(form.title.to_owned()),
                text: Set(form.text.to_owned()),
            }
            .save(txn)
            .await
            .expect("could not edit post");

            Ok(())
        })
    })
    .await
    .unwrap();

    Flash::success(Redirect::to("/"), "Post successfully edited.")
}

#[get("/?<page>&<posts_per_page>")]
async fn list(
    conn: Connection<'_, Db>,
    page: Option<usize>,
    posts_per_page: Option<usize>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let db = conn.into_inner();

    // Set page number and items per page
    let page = page.unwrap_or(1);
    let posts_per_page = posts_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);
    if page == 0 {
        panic!("Page number cannot be zero");
    }

    // Setup paginator
    let paginator = Post::find()
        .order_by_asc(post::Column::Id)
        .paginate(db, posts_per_page);
    let num_pages = paginator.num_pages().await.ok().unwrap();

    // Fetch paginated posts
    let posts = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retrieve posts");

    Template::render(
        "index",
        json! ({
            "page": page,
            "posts_per_page": posts_per_page,
            "num_pages": num_pages,
            "posts": posts,
            "flash": flash.map(FlashMessage::into_inner),
        }),
    )
}

#[get("/<id>")]
async fn edit(conn: Connection<'_, Db>, id: i32) -> Template {
    let db = conn.into_inner();

    let post: Option<post::Model> = Post::find_by_id(id)
        .one(db)
        .await
        .expect("could not find post");

    Template::render(
        "edit",
        json! ({
            "post": post,
        }),
    )
}

#[delete("/<id>")]
async fn delete(conn: Connection<'_, Db>, id: i32) -> Flash<Redirect> {
    let db = conn.into_inner();

    let post: post::ActiveModel = Post::find_by_id(id).one(db).await.unwrap().unwrap().into();

    post.delete(db).await.unwrap();

    Flash::success(Redirect::to("/"), "Post successfully deleted.")
}

#[delete("/")]
async fn destroy(conn: Connection<'_, Db>) -> Result<(), rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();

    Post::delete_many().exec(db).await.unwrap();
    Ok(())
}
