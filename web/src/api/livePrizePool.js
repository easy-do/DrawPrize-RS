import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const getLivePrizePoolList = () => get("/api/live_prize_pool/list");
export const getLivePrizePoolSelectList = () => get("/un-auth-api/live_prize_pool/select_list");
export const getLivePrizePoolInfo = (id) => get("/api/live_prize_pool/info/"+id);
export const getLivePrizePoolPage = (param) => postRequestBody("/api/live_prize_pool/page",param);
export const updateLivePrizePool = (param) => postRequestBody("/api/live_prize_pool/update",param);
export const drawLivePrizePool = (live_id,draw_num) => get("/api/live_prize_pool/draw/"+live_id+"/"+draw_num);
export const topDraw = () => get("/un-auth-api/live_prize_pool/top_draw");
export const getPrizeItemList = (live_id) => get("/un-auth-api/live_prize_pool/prize_item_list/"+live_id);
export const getDrawHistory = () => get("/un-auth-api/live_prize_pool/draw_history");
export const getPoolDrawCount = (live_id) => get("/un-auth-api/live_prize_pool/pool_draw_count/"+live_id);
export const getUserDrawRemainingTimes = (live_id) => get("/api/live_prize_pool/user_draw_remaining_times/"+live_id);

