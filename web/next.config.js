/* eslint-disable @typescript-eslint/no-var-requires */
/** @type {import('next').NextConfig} */
const path = require('path');
const withLess = require('next-with-less');
const withTM = require('next-transpile-modules')([
  '@arco-design/web-react',
  '@arco-themes/react-arco-pro',
]);

const setting = require("./src/settings.json");

module.exports = withLess(
  withTM({
    lessLoaderOptions: {
      lessOptions: {
        modifyVars: {
          'arcoblue-6': setting.themeColor,
        },
      },
    },
    webpack: (config) => {
      config.module.rules.push({
        test: /\.svg$/,
        use: ['@svgr/webpack'],
      });

      config.resolve.alias['@/assets'] = path.resolve(
        __dirname,
        './src/public/assets'
      );
      config.resolve.alias['@'] = path.resolve(__dirname, './src');

      return config;
    },
    async redirects() {
      return [
        {
          source: '/',
          destination: '/static/home.html', //生产环境 需要配置成静态文件，否则会报404错误
          // destination: '/home', //开发环境
          permanent: true,
        },
      ];
    },
    async rewrites() {
      return [
        {
          source: '/api/:path*',
          destination: 'http://localhost:8080/api/:path*', 
        },
        {
          source: '/un-auth-api/:path*',
          destination: 'http://localhost:8080/un-auth-api/:path*', 
        },
      ];
    },
    // assetPrefix: "/static/",  // 开发环境禁用 生产环境打开 需要配置成静态文件，否则会报404错误 
    // basePath: "/static",  //开发环境禁用 生产环境打开 需要配置成静态文件，否则会报404错误
    pageExtensions: ['tsx'],
  })
);
