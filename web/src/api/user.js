import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const getUserList = () => get("/api/user/list");
export const removeUser = (id) => get("/api/user/delete/"+id);
export const getUserInfo = (id) => get("/api/user/info/"+id);
export const getUserPage = (param) => postRequestBody("/api/user/page",param);
export const addUser = (param) => postRequestBody("/api/user/add",param);
export const updateUser = (param) => postRequestBody("/api/user/update",param);
export const resetPassword = (param) => postRequestBody("/api/user/reset_password",param);
export const getUserRole = (id) => get("/api/user/get_role/"+id);
export const setUerRoles = (param) => postRequestBody("/api/user/set_role",param);

