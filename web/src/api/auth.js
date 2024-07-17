import { get,postRequestBody } from "../utils/request"

/** 授权相关 */
export const loginRequst = (param) => postRequestBody("/un-auth-api/auth/login",param);
export const getUserInfo = () => get("/api/auth/user_info");
export const resetPassword = (param) => postRequestBody("/api/auth/reset_password",param);
export const logoutRequest = () => get("/api/auth/logout");
export const userMenu = (authorizationCode) => get("/api/auth/user_menu/" + authorizationCode )
export const sendEmail = (email) => get("/uc/un-auth-api/send_email?email="+email);
export const getCaptchaV1 = () => get("/un-auth-api/auth/captcha_v1");
export const getCaptchaV2 = () => get("/un-auth-api/auth/captcha_v2");
export const registerRequest = (param) => postRequestBody("/un-auth-api/auth/register",param);

