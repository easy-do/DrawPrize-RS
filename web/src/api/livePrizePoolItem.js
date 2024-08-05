import { get, postRequestBody } from "@/utils/request"

/** 授权相关 */
export const getLivePrizePoolItemList = () => get("/api/live_prize_pool_item/list");
export const getLivePrizePoolItemInfo = (id) => get("/api/live_prize_pool_item/info/" + id);
export const getLivePrizePoolItemPage = (param) => postRequestBody("/api/live_prize_pool_item/page", param);
export const updateLivePrizePoolItem = (param) => postRequestBody("/api/live_prize_pool_item/update", param);
export const removeLivePrizePoolItem = (id) => get("/api/live_prize_pool_item/delete/" + id);
export const addLivePrizePoolItem = (live_id,item_id) => get("/api/live_prize_pool_item/add/"+live_id+"/"+item_id);

