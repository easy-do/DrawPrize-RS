import { get, postRequestBody } from "@/utils/request"

/** 授权相关 */
export const getPrizePoolItemList = () => get("/api/prize_pool_item/list");
export const removePrizePoolItem = (id) => get("/api/prize_pool_item/delete/" + id);
export const getPrizePoolItemInfo = (id) => get("/api/prize_pool_item/info/" + id);
export const getPrizePoolItemPage = (param) => postRequestBody("/api/prize_pool_item/page", param);
export const addPrizePoolItem = (param) => postRequestBody("/api/prize_pool_item/add", param);
export const updatePrizePoolItem = (param) => postRequestBody("/api/prize_pool_item/update", param);
export const getPrizePoolItemBtPoolId = (pool_id) => get("/api/prize_pool_item/get_by_pool_id/" + pool_id);
