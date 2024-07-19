import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const getLivePrizePoolList = () => get("/api/live_prize_pool/list");
export const getLivePrizePoolInfo = (id) => get("/api/live_prize_pool/info/"+id);
export const getLivePrizePoolPage = (param) => postRequestBody("/api/live_prize_pool/page",param);
export const updateLivePrizePool = (param) => postRequestBody("/api/live_prize_pool/update",param);

