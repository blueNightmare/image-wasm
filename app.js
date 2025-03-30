const Koa = require('koa');
const serve = require('koa-static');
const path = require('path');

const app = new Koa();

// 静态资源目录
const staticDir = path.join(__dirname);

// 使用 koa-static 中间件来提供静态资源服务
app.use(serve(staticDir));

const port = 3000;
app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
    