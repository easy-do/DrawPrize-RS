import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const removeCdk = (param) => postRequestBody("/api/cdk/delete/",param);
export const getCdkPage = (param) => postRequestBody("/api/cdk/page",param);
export const getCdkInfo = (id) => get("/api/cdk/info/"+id);
export const addCdk = (param) => postRequestBody("/api/cdk/add",param);
export const updateCdk = (param) => postRequestBody("/api/cdk/update",param);
export const exportCdk = (param) => postRequestBody("/api/cdk/export",param);
export const requetUseCdk = (param) => postRequestBody("/api/cdk/use_cdk",param);

