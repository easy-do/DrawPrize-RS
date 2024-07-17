use std::str::FromStr;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, Iterable, NotSet, PaginatorTrait, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;

use common::error::MyError;
use common::page::{PageResult};
use entity::prelude::User;
use entity::user;
use model::auth::RegisterModel;
use model::user::{CreateUser, UserPage};
use security::bcrypt::hash_context;
use crate::manager::role_manager;

pub async fn get_user_list(db: &DbConn) -> Result<Vec<user::Model>, MyError> {
    let res = User::find()
        .select_only()
        .columns(user::Column::iter().filter(|col| match col {
            user::Column::Id => true,
            user::Column::UserName => true,
            user::Column::NickName => true,
            _ => false,
        }))
        .all(db).await?;
    Ok(res)
}

pub async fn create_user_data(db: &DbConn, form: CreateUser) -> Result<i64, MyError> {
    let user_name = form.user_name.ok_or(MyError::ServerError("用户名不能为空".to_string()))?;
    check_user_name_exist(db,user_name.clone()).await?;
    let nick_name = form.nick_name.ok_or(MyError::ServerError("昵称不能为空".to_string()))?;
    check_user_nick_name_exist(db,nick_name.clone()).await?;
    let email = form.email.ok_or(MyError::ServerError("邮箱不能为空".to_string()))?;
    check_user_email_exist(db,email.clone()).await?;
    let password = form.password.ok_or(MyError::ServerError("密码不能为空".to_string()))?;
    let password = hash_context(password)?;
    let model = user::ActiveModel {
        id: NotSet,
        user_name: Set(Some(user_name)),
        password: Set(Some(password)),
        nick_name: Set(Some(nick_name)),
        email: Set(Some(email)),
        email_status: Set(Some(false)),
        status: Set(Some(true)),
        create_time: Set(Option::from(Local::now().naive_local())),
        update_time: NotSet,
        ..Default::default()
    }.insert(db).await?;
    role_manager::set_user_role(db, model.id, vec![2]).await?;
    Ok(model.id)
}

pub async fn register_user_data(db: &DbConn, form: RegisterModel) -> Result<bool, MyError> {
    let user_name = form.user_name.ok_or(MyError::ServerError("用户名不能为空".to_string()))?;
    check_user_name_exist(db, user_name.clone()).await?;
    let email = form.email.ok_or(MyError::ServerError("邮箱不能为空".to_string()))?;
    check_user_email_exist(db, email.clone()).await?;
    let nick_name = form.nick_name.ok_or(MyError::ServerError("昵称不能为空".to_string()))?;
    check_user_nick_name_exist(db,nick_name.clone()).await?;
    let password = form.password.ok_or(MyError::ServerError("密码不能为空".to_string()))?;
    let password = hash_context(password)?;
    let model = user::ActiveModel {
        id: NotSet,
        user_name: Set(Some(user_name)),
        password: Set(Some(password)),
        nick_name: Set(Some(nick_name)),
        email: Set(Some(email)),
        email_status: Set(Some(false)),
        status: Set(Some(true)),
        create_time: Set(Some(Local::now().naive_local())),
        update_time: NotSet,
        last_login_time: NotSet,
    }.insert(db).await;
    let res = model.is_ok();
    if res {
        role_manager::set_user_role(db, model.unwrap().id, vec![2]).await?;
    }
    Ok(res)
}

pub async fn update_user_data(db: &DbConn, form: user::Model) -> Result<i64, MyError> {
    let entity = User::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    let mut entity: user::ActiveModel = entity.into();
    if form.nick_name.is_some() {
        entity.nick_name = Set(Option::from(check_user_nick_name_and_id_not_in(db, form.nick_name.unwrap().clone(), form.id.clone()).await?));
    }
    if form.status.is_some() {
        entity.status = Set(form.status);
    }
    if form.email_status.is_some() {
        entity.email_status = Set(form.email_status);
    }
    if form.user_name.is_some() {
        entity.user_name = Set(Option::from(check_user_name_exist_and_id_not_in(db, form.user_name.unwrap().clone(), form.id.clone()).await?));
    }
    if form.email.is_some() {
        entity.email = Set(Option::from(check_user_email_exist_and_id_not_in(db, form.email.unwrap().clone(), form.id.clone()).await?));
    }
    entity.update_time = Set(Some(Local::now().naive_local()));
    let result = entity.update(db).await?;
    Ok(result.id)

}

pub async fn delete_user_data(db: &DbConn, user_id: i64) -> Result<bool, MyError> {
    let res = User::delete_by_id(user_id).exec(db).await?;
    Ok(res.rows_affected == 1)
}

pub async fn get_user_data(db: &DbConn, user_id: i64) -> Result<Option<user::Model>, MyError> {
    let res = User::find_by_id(user_id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, user_page: UserPage) -> Result<PageResult<user::Model>, MyError> {
    let mut page_data = user_page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();

    let mut find = User::find();

    find = find.select_only().columns(user::Column::iter().filter(|col| match col {
        user::Column::Password => false,
        _ => true,
    }));

    let id = user_page.id;
    if id.is_some() {
        find = find.filter(user::Column::Id.eq(id.unwrap()))
    }

    let user_name = user_page.user_name;
    if user_name.is_some() {
        find = find.filter(user::Column::UserName.like(format!("%{}%",user_name.unwrap())))
    }

    let nick_name = user_page.nick_name;
    if nick_name.is_some() {
        find = find.filter(user::Column::NickName.like(format!("%{}%",nick_name.unwrap())))
    }

    let status = user_page.status;
    if status.is_some() {
        find = find.filter(user::Column::Status.eq(status.unwrap()))
    }

    let create_time = user_page.create_time;
    if create_time.is_some() {
        let create_time = create_time.unwrap();
        find = find.filter(user::Column::CreateTime.between(create_time[0].clone(),create_time[1].clone()))
    }

    let last_login_time = user_page.last_login_time;
    if last_login_time.is_some() {
        let last_login_time = last_login_time.unwrap();
        find = find.filter(user::Column::LastLoginTime.between(last_login_time[0].clone(),last_login_time[1].clone()))
    }

    let sorter = page_data.sorter;
    if sorter.is_some() {
        let sorter = sorter.unwrap();
        let field = user::Column::from_str(sorter.field.as_str()).or_else(|e| {
            Err(MyError::DBError(format!("获取排序字段失败：{}",e.to_string())))
        })?;
        find = find.order_by(field,sorter.order());
    }

    let paginator = find.paginate(db, page_data.page_size);

    //分页查询
    let record = paginator.fetch_page(page_data.page - 1).await?;
    //总条数
    let total = paginator.num_items().await?;
    //返回分页结果
    Ok(PageResult {
        page: page_data.page,
        page_size: page_data.page_size,
        total,
        record,
    })
}

pub async fn exist_by_name(db: &DbConn, user_name: String) -> Result<bool, MyError> {
    let exist = User::find().filter(user::Column::UserName.eq(user_name))
        .all(db).await?.len() > 0;
    Ok(exist)
}
pub async fn exist_by_email(db: &DbConn, email: String) -> Result<bool, MyError> {
    let exist = User::find().filter(user::Column::Email.eq(email))
        .all(db).await?.len() > 0;
    Ok(exist)
}
pub async fn exist_by_nick_name(db: &DbConn, nick_name: String) -> Result<bool, MyError> {
    let exist = User::find().filter(user::Column::NickName.eq(nick_name))
        .all(db).await?.len() > 0;
    Ok(exist)
}

pub async fn exist_by_name_and_id_not_in(db: &DbConn, user_name: String, uid: i64) -> Result<bool,MyError> {
    Ok(User::find()
        .filter(user::Column::UserName.eq(user_name))
        .filter(user::Column::Id.ne(uid))
        .all(db).await?.len() > 0)
}

pub async fn exist_by_email_and_id_not_in(db: &DbConn, email: String, uid: i64) -> Result<bool,MyError>  {
    Ok(User::find()
        .filter(user::Column::Email.eq(email))
        .filter(user::Column::Id.ne(uid))
        .all(db).await?.len() > 0)
}

pub async fn exist_by_nick_name_and_id_not_in(db: &DbConn, nick_name: String, uid: i64) -> Result<bool,MyError>  {
    Ok(User::find()
        .filter(user::Column::NickName.eq(nick_name))
        .filter(user::Column::Id.ne(uid))
        .all(db).await?.len() > 0)
}

pub async fn check_user_name_exist(db: &DbConn, user_name: String) -> Result<String, MyError> {
    if exist_by_name(db,user_name.clone()).await? {
        Err(MyError::ServerError(format!("用户名[{}]已存在",user_name)))
    } else {
        Ok(user_name)
    }
}

pub async fn check_user_email_exist(db: &DbConn, email: String) -> Result<String, MyError> {
    if exist_by_email(db,email.clone()).await? {
        Err(MyError::ServerError(format!("邮箱[{}]已存在",email)))
    } else {
        Ok(email)
    }
}
pub async fn check_user_nick_name_exist(db: &DbConn, nick_name: String) -> Result<String, MyError> {
    if exist_by_nick_name(db,nick_name.clone()).await? {
        Err(MyError::ServerError(format!("昵称[{}]已存在",nick_name)))
    } else {
        Ok(nick_name)
    }
}

pub async fn check_user_name_exist_and_id_not_in(db: &DbConn, user_name: String, uid: i64) -> Result<String, MyError> {
    if exist_by_name_and_id_not_in(db, user_name.clone(),uid).await? {
        Err(MyError::ServerError(format!("用户名[{}]已存在",user_name)))
    } else {
        Ok(user_name.clone())
    }
}

pub async fn check_user_email_exist_and_id_not_in(db: &DbConn, email: String, uid: i64) -> Result<String, MyError> {
    if exist_by_email_and_id_not_in(db,email.clone(),uid).await? {
        Err(MyError::ServerError(format!("邮箱[{}]已存在",email)))
    } else {
        Ok(email.clone())
    }
}
pub async fn check_user_nick_name_and_id_not_in(db: &DbConn, nick_name: String, uid: i64) -> Result<String, MyError> {
    if exist_by_nick_name_and_id_not_in(db,nick_name.clone(),uid).await? {
        Err(MyError::ServerError(format!("昵称[{}]已存在",nick_name)))
    } else {
        Ok(nick_name.clone())
    }
}


pub async fn get_user_by_user_name(db: &DbConn, user_name: String) -> Result<Option<user::Model>, MyError> {
    let user = User::find().filter(user::Column::UserName.eq(user_name)).one(db).await?;
    Ok(user)
}

pub async fn reset_password(db: &DbConn, uid: i64, password: String) -> Result<bool, MyError> {
    user::ActiveModel {
        id: Set(uid),
        user_name: NotSet,
        password: Set(Some(password)),
        nick_name: NotSet,
        email: NotSet,
        email_status: NotSet,
        status: NotSet,
        create_time: Set(Some(Local::now().naive_local())),
        update_time: NotSet,
        last_login_time: NotSet,
    }.update(db).await?;
    Ok(true)
}

pub async fn update_login_time(db: &DbConn, uid: i64) -> Result<bool, MyError> {
    let entity = User::find_by_id(uid).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", uid)))?;
    let mut entity: user::ActiveModel = entity.into();
    entity.last_login_time = Set(Some(Local::now().naive_local()));
    entity.update(db).await?;
    Ok(true)
}

pub async fn get_user_by_email(db: &DbConn, email: &String) -> Result<Option<user::Model>, MyError> {
    let user = User::find().filter(user::Column::Email.eq(email)).one(db).await?;
    Ok(user)
}