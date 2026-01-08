# subtitles

人人影视字幕数据搜索

数据来源：https://github.com/qundao/mirror-yyets-subtitles

## 特征

已经将 MySQL 数据转换为 kv 数据库 fjall，无须安装 MySQL。

## 使用

1. 下载并解压字幕文件，并将其文件夹路径配置到 `config.toml` 中的 `subtitle_dir`。
2. 编译并运行程序：

```bash
git clone https://github.com/cncases/subtitles
cd subtitles
cargo build --release
./target/release/subtitles
```

