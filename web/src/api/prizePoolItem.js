import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const getPrizePoolList = () => get("/api/prize_pool_item/list");
export const removePrizePool= (id) => get("/api/prize_pool_item/delete/"+id);
export const getPrizePoolInfo = (id) => get("/api/prize_pool_item/info/"+id);
export const getPrizePoolPage = (param) => postRequestBody("/api/prize_pool_item/page",param);
export const addPrizePool = (param) => postRequestBody("/api/prize_pool_item/add",param);
export const updatePrizePool = (param) => postRequestBody("/api/prize_pool_item/update",param);

