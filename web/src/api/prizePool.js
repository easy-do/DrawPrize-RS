import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const getPrizePoolList = () => get("/api/prize_pool/list");
export const removePrizePool= (id) => get("/api/prize_pool/delete/"+id);
export const getPrizePoolInfo = (id) => get("/api/prize_pool/info/"+id);
export const getPrizePoolPage = (param) => postRequestBody("/api/prize_pool/page",param);
export const addPrizePool = (param) => postRequestBody("/api/prize_pool/add",param);
export const updatePrizePool = (param) => postRequestBody("/api/prize_pool/update",param);
export const createLivePool = (id) => get("/api/prize_pool/create_live_pool/"+id);

