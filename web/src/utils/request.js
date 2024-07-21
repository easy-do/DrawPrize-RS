import axios from "axios";
import { Component } from "react"; 3
import cookie from 'react-cookies'
import { Notification } from '@arco-design/web-react';
let baseUrl = "";

// 请求前拦截
axios.interceptors.request.use(
  config => {
    return config;
  },
  err => {
    Notification.warning({ content: '请求超时.', duration: 3000 })
    return Promise.reject(err);
  }
);

// 返回后拦截
axios.interceptors.response.use(
  data => {
    console.log("axios.interceptors.response",data);
    const {code,success,message} = data.data;
    //mock的接口数据直接放过
    if(code == undefined && success == undefined && message == undefined){
      return data;
    }
    if(code == 200 && success){
      
      return data;
    }
    if (code === 401) {
      Notification.warning({ content: message, duration: 3000 })
      cookie.remove('Authorization');
      localStorage.clear()
      sessionStorage.clear()
      // window.location.pathname = '/home' // 开发环境
      window.location.pathname = '/static/home.html';//生产环境 需要配置成静态文件，否则会报404错误
    } else {
      Notification.warning({ content: message, duration: 3000 })
    }
  },
  err => {
    if (err.response.status && err.response.status !== 200) {
      Notification.warning({ content: err.response.status + '请求异常.', duration: 3000 })
    }
    return Promise.reject(err);
  }
);


// @RequestBody请求
export const postRequestBody = (url, params) => {
  return axios({
    method: "post",
    url: `${baseUrl}${url}`,
    data: params,
    headers: {
      "Authorization": localStorage.getItem("Authorization") ? localStorage.getItem("Authorization") : "",
      "Content-Type": "application/json",
      "accept-language": localStorage.getItem("arco-lang") ? localStorage.getItem("arco-lang") : "zh-CN",
      charset: "utf-8"
    }
  });
};

// @RequsetParam请求
export const postRequestParam = (url, params) => {
  return axios({
    method: "post",
    url: `${baseUrl}${url}`,
    data: params,
    transformRequest: [
      function (data) {
        let ret = "";
        for (let it in data) {
          ret +=
            encodeURIComponent(it) + "=" + encodeURIComponent(data[it]) + "&";
        }
        return ret;
      }
    ],
    headers: {
      "Authorization": localStorage.getItem("Authorization") ? localStorage.getItem("Authorization") : "",
      "Content-Type": "application/x-www-form-urlencoded",
      "accept-language": localStorage.getItem("arco-lang") ? localStorage.getItem("arco-lang") : "zh-CN",
    }
  });
};

// @form-data请求
export const postFormData = (url, params) => {
  return axios({
    method: "post",
    url: `${baseUrl}${url}`,
    data: params,
    headers: {
      "Authorization": localStorage.getItem("Authorization") ? localStorage.getItem("Authorization") : "",
      "Content-Type": "multipart/form-data",
      "accept-language": localStorage.getItem("arco-lang") ? localStorage.getItem("arco-lang") : "zh-CN",
    }
  });
};

export const get = url => {
  return axios({
    method: "get",
    url: `${baseUrl}${url}`,
    headers: {
      "Authorization": localStorage.getItem("Authorization") ? localStorage.getItem("Authorization") : "",
      "Content-Type": "text/plain",
      "accept-language": localStorage.getItem("arco-lang") ? localStorage.getItem("arco-lang") : "zh-CN",
    }
  });
};

export const multiple = function (requsetArray, callback) {
  axios.all(requsetArray).then(axios.spread(callback));
};

Component.prototype.get = get;
Component.prototype.postRequestBody = postRequestBody;
Component.prototype.postRequestParam = postRequestParam;
Component.prototype.postFormData = postFormData;
Component.prototype.multiple = multiple;


const mimeMap = {
  xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet;charset=utf-8',
  zip: 'application/zip;charset=utf-8',
  word: 'application/msword;charset=utf-8',
  pdf: 'application/pdf;charset=utf-8'
}

export function downLoadGet(str, type) {
  var url = baseUrl + str
  axios({
    method: 'get',
    url: url,
    responseType: 'blob',
    headers: {
      "Authorization": localStorage.getItem("Authorization") ? localStorage.getItem("Authorization") : ""
    }
  }).then(res => {
    if(type == 'zip'){
      resolveBlob(res, mimeMap.zip)
    }
    if(type == 'xlsx'){
      resolveBlob(res, mimeMap.xlsx)
    }
    if(type == 'word'){
      resolveBlob(res, mimeMap.word)
    }
    if(type == 'pdf'){
      resolveBlob(res, mimeMap.pdf)
    }
  })
}

export function downLoadPost(str, param, type) {
  var url = baseUrl + str
  axios({
    method: 'post',
    url: url,
    responseType: 'blob',
    headers: {
      "Authorization": localStorage.getItem("Authorization") ? localStorage.getItem("Authorization") : ""
    },
    data: param
  }).then(res => {
    if(type == 'zip'){
      resolveBlob(res, mimeMap.zip)
    }
    if(type == 'xlsx'){
      resolveBlob(res, mimeMap.xlsx)
    }
    if(type == 'word'){
      resolveBlob(res, mimeMap.word)
    }
    if(type == 'pdf'){
      resolveBlob(res, mimeMap.pdf)
    }
  })
}

/**
 * 解析blob响应内容并下载
 * @param {*} res blob响应内容
 * @param {String} mimeType MIME类型
 */
export function resolveBlob(res, mimeType) {
  console.info(res);
  const downloadElement = document.createElement('a');
  var blob = new Blob([res.data], { type: mimeType });
  console.info(blob);
  // //从response的headers中获取filename, 后端response.setHeader("Content-disposition", "attachment; filename=xxxx.docx") 设置的文件名;
  var patt = new RegExp('filename=([^;]+\\.[^\\.;]+);*');
  var contentDisposition = decodeURI(res.headers['content-disposition']);
  var result = patt.exec(contentDisposition);
  var fileName = decodeURI(result[1]);
  // fileName = fileName.replace(/\"/g, '')
  var href = URL.createObjectURL(blob);
  downloadElement.href = href;
  downloadElement.download = fileName; // 设置下载文件名称
  downloadElement.style.display = 'none';
  document.body.appendChild(downloadElement);
  downloadElement.click();
  document.body.removeChild(downloadElement); //下载完成移除元素
  window.URL.revokeObjectURL(href); //释放掉blob对象
}