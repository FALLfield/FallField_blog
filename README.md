# FallField 个人博客

这是一个使用Rust和Rocket框架构建的个人博客系统。

## 本地开发

1. 安装Rust和Cargo
2. 克隆项目
3. 运行命令：`cargo run`
4. 访问 http://localhost:8000

## 部署到GitHub

本项目可以通过GitHub Actions部署到Fly.io。步骤如下：

1. 在GitHub上创建仓库
2. 将代码推送到仓库
3. 在Fly.io上注册账号并安装CLI工具
4. 运行 `flyctl auth token` 获取API令牌
5. 在GitHub仓库设置中添加Secret：`FLY_API_TOKEN`
6. 推送代码到main分支即可触发自动部署

### 手动部署

也可以手动部署：

```bash
# 首次部署
flyctl launch

# 后续更新
flyctl deploy
```

## 项目结构

- `src/`: 源代码
- `templates/`: 页面模板
- `static/`: 静态资源
- `migrations/`: 数据库迁移脚本 