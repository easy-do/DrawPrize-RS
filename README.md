# DrawPrize-RS

#### 介绍 desc
一个使用rust实现的抽奖系统 ，A lottery system implemented using Rust

#### 软件架构 Software architecture
Rust
actix
sea-orm
nextjs
arco design

#### 开发 develop

1.  clone 
2.  install rust | install nodejs >=18.x |
3.  start web : cd web | yarn install | yarn run dev | http://localhost:3000
4.  start api : cd rust-src | cargo build | cargo run main.rs | http://localhost:8080


#### 编译 compile 

1.  clone
2.  cd web | yarn run install | yarn run build | yarn run export 
3.  cp out/* xxx/static
4.  cd ../rust-src  | cargo build --release
5.  cp target/release/DrawPrize-RS  xxx
6.  cd xxx 
7.  ./DrawPrize-RS

#### 使用说明 instructions

1.  http://localhost:8080/static/index.html
2.  admin / admin




