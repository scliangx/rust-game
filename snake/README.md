# 贪吃蛇游戏 (Snake Game)

使用 Rust 和 Bevy 游戏引擎实现的经典贪吃蛇游戏。

## 功能特性

- 经典的贪吃蛇玩法
- 流畅的移动和碰撞检测
- 分数系统
- 游戏结束检测
- 模块化代码结构

## 控制方式

- **方向键**：控制蛇的移动方向
  - ↑ 上
  - ↓ 下
  - ← 左
  - → 右

## 游戏规则

1. 使用方向键控制蛇的移动
2. 吃到红色食物可以增加分数并使蛇变长
3. 避免撞到墙壁或蛇的身体
4. 游戏结束后显示最终分数

## 项目结构

```
snake/
├── src/
│   ├── main.rs              # 程序入口
│   ├── components/          # ECS 组件
│   │   ├── mod.rs
│   │   ├── position.rs      # 位置和实体类型组件
│   │   └── direction.rs     # 方向组件
│   ├── constants/           # 游戏常量
│   │   └── mod.rs
│   ├── plugins/             # Bevy 插件
│   │   ├── mod.rs
│   │   └── snake_plugin.rs  # 主游戏插件
│   ├── resources/           # 游戏资源
│   │   ├── mod.rs
│   │   └── game_resources.rs # 游戏状态资源
│   └── systems/             # 游戏系统
│       ├── mod.rs
│       ├── setup_system.rs      # 初始化系统
│       ├── input_system.rs      # 输入处理系统
│       ├── move_snake_system.rs # 蛇移动系统
│       ├── collision_system.rs  # 碰撞检测系统
│       ├── food_system.rs       # 食物系统
│       └── game_over_system.rs  # 游戏结束系统
├── Cargo.toml            # 项目配置
└── README.md             # 项目说明
```

## 运行游戏

### 前置要求

- 安装 Rust (推荐使用 rustup)
- 安装 Bevy 的系统依赖

### 编译和运行

```bash
# 进入项目目录
cd snake

# 编译项目
cargo build

# 运行游戏
cargo run
```

## 技术栈

- **Rust** - 系统编程语言
- **Bevy** - ECS 游戏引擎
- **rand** - 随机数生成

## 开发说明

项目采用模块化设计，每个功能模块独立组织：

- **components**: 定义游戏实体的数据结构
- **systems**: 实现游戏逻辑和行为
- **resources**: 管理全局游戏状态
- **plugins**: 组织和配置系统
- **constants**: 存储游戏配置常量

## 许可证

本项目仅用于学习和演示目的。