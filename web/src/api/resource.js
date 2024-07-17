import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const getResourceList = () => get("/api/resource/list");
export const getResourceTree = () => get("/api/resource/tree");
export const removeResource= (id) => get("/api/resource/delete/"+id);
export const getResourceInfo = (id) => get("/api/resource/info/"+id);
export const getResourcePage = (param) => postRequestBody("/api/resource/page",param);
export const addResource = (param) => postRequestBody("/api/resource/add",param);
export const updateResource = (param) => postRequestBody("/api/resource/update",param);

