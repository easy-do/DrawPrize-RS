import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const getRoleList = () => get("/api/role/list");
export const removeRole = (id) => get("/api/role/delete/"+id);
export const getRoleInfo = (id) => get("/api/role/info/"+id);
export const getRolePage = (param) => postRequestBody("/api/role/page",param);
export const addRole = (param) => postRequestBody("/api/role/add",param);
export const updateRole = (param) => postRequestBody("/api/role/update",param);
export const getRoleResource = (id) => get("/api/role/get_resource/"+id);
export const setRoleResource = (param) => postRequestBody("/api/role/set_resource",param);

