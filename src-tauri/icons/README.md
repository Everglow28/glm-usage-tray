# 托盘图标说明

## 当前状态

当前包含 `icon.svg` 作为图标源文件。

## 生成 PNG 图标

请使用以下方式之一生成 256x256 的 PNG 图标：

### 方法 1: 使用在线工具
访问 https://cloudconvert.com/svg-to-png 上传 `icon.svg` 并转换为 PNG。

### 方法 2: 使用 ImageMagick
```bash
magick -background none icon.svg -resize 256x256 icon.png
```

### 方法 3: 使用 Inkscape
1. 打开 Inkscape
2. 打开 `icon.svg`
3. 导出为 PNG (256x256)
4. 保存为 `icon.png`

## 自定义图标

你可以替换 `icon.svg` 为自定义图标，然后按上述方法生成 PNG 版本。
