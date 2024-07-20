import { get, postRequestBody } from "@/utils/request"

/** 授权相关 */
export const userDrawHistoryPage = (param) => postRequestBody("/api/draw_history/user_page", param);

