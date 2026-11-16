// 构建前把 NSIS 自带的 Korean.nlf 换成用 Malgun Gothic 的版本。
//
// 原因：NSIS 的韩语语言文件指定字体 Gulim，非韩文版 Windows 没装这个字体，
// 韩语界面的正文和按钮会全变成方块（标题能显示，正文不能）。
// Malgun Gothic 是 Windows 自带的韩文字体，覆盖率足够。
//
// 由 tauri.conf.json 的 build.beforeBundleCommand 调用；
// 路径按脚本自身位置解析，不依赖当前工作目录。
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const src = path.resolve(here, '..', 'src-tauri', 'nsis', 'Korean.nlf');
const nsisRoot = path.join(os.homedir(), 'AppData', 'Local', 'tauri', 'NSIS');

if (!fs.existsSync(src)) {
  console.log('[patch-nsis] 跳过：没有找到 src-tauri/nsis/Korean.nlf');
  process.exit(0);
}

if (!fs.existsSync(nsisRoot)) {
  console.log('[patch-nsis] 跳过：NSIS 工具链还没下载（首次构建完成后请再跑一次 npm run build）');
  process.exit(0);
}

let patched = 0;
let found = 0;
const walk = (dir) => {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walk(full);
    } else if (entry.name.toLowerCase() === 'korean.nlf') {
      found++;
      const current = fs.readFileSync(full);
      const wanted = fs.readFileSync(src);
      if (!current.equals(wanted)) {
        fs.copyFileSync(src, full);
        patched++;
      }
    }
  }
};
walk(nsisRoot);

console.log(`[patch-nsis] Korean.nlf 目标 ${found} 个，已替换 ${patched} 个`);
