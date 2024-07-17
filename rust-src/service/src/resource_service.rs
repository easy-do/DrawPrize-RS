use std::collections::HashMap;
use sea_orm::{DatabaseConnection, DbConn};

use common::error::MyError;
use common::page::PageResult;
use entity::resource;
use model::resource::{CreateResource, ResourcePage, ResourceTree};
use crate::auth_service::build_all_resource;

use crate::manager::resource_manager;

pub async fn list(db: &DbConn) -> Result<Vec<resource::Model>, MyError> {
    Ok(resource_manager::get_resource_list(db).await?)
}

pub async fn add(db: &DbConn, form: CreateResource) -> Result<i64, MyError> {
    Ok(resource_manager::create_resource_data(db, form).await?)
}

pub async fn update(db: &DbConn, form: resource::Model) -> Result<i64, MyError> {
    Ok(resource_manager::update_resource_data(db, form).await?)
}

pub async fn delete(db: &DbConn, resource_id: i64) -> Result<bool, MyError> {
    Ok(resource_manager::delete_resource_data(db, resource_id).await?)
}

pub async fn info(db: &DbConn, resource_id: i64) -> Result<resource::Model, MyError> {
    let resource = resource_manager::get_resource_data(db, resource_id).await?
        .ok_or(MyError::ServerError(format!("resource [{:?}] does not exist", resource_id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, resource_page: ResourcePage) -> Result<PageResult<resource::Model>, MyError> {
    resource_manager::page(db, resource_page).await
}

pub async fn resource_tree(db: &DatabaseConnection) -> Result<Vec<ResourceTree>, MyError> {
    let mut result_vec = Vec::new();
    let list = resource_manager::get_resource_all_list(db).await?;
    let all_resource_map = build_all_resource(&list)?;
    let root_resource = all_resource_map.0;
    let parent_key_map: HashMap<i64, Vec<resource::Model>> = all_resource_map.1;
    //遍历所有根节点，依次构建每个节点和对应的子节点
    for resource in root_resource {
        let child_menu = build_child_resource_tree(&resource, &parent_key_map)?;
        //构建根节点
        result_vec.push(ResourceTree {
            parent_id: resource.parent_id,
            key: resource.id,
            title: resource.resource_name.unwrap(),
            children: child_menu,
        })
    }
    Ok(result_vec)
}


fn build_child_resource_tree(parent_resource: &resource::Model, parent_key_map: &HashMap<i64, Vec<resource::Model>>) -> Result<Vec<ResourceTree>, MyError> {
    let mut children_tree_list = Vec::new();
    let child_resources = parent_key_map.get(&parent_resource.id);
    match child_resources {
        //如果存在子节点则继续遍历
        Some(child_resources) => {
            //构建子节点
            for child_resource in child_resources {
                let tmp_child_resource = child_resource.clone();
                let child = build_child_resource_tree(&child_resource, parent_key_map)?;
                let children_tree = ResourceTree {
                    parent_id: tmp_child_resource.parent_id,
                    key: tmp_child_resource.id,
                    title: tmp_child_resource.resource_name.unwrap(),
                    children: child,
                };
                children_tree_list.push(children_tree)
            }
        }
        _ => {}
    }
    //返回构建好的节点
    Ok(children_tree_list)
}