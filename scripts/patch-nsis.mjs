// 构建前把 NSIS 自带的 Korean.nlf 换成用 Malgun Gothic 的版本。
// 原因：NSIS 的韩语语言文件指定字体 Gulim，非韩文版 Windows 没装这个字体，
// 韩语界面里的控件会全变成方块。Malgun Gothic 是 Windows 自带的。
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';

const src = path.resolve('src-tauri/nsis/Korean.nlf');
const nsisRoot = path.join(os.homedir(), 'AppData', 'Local', 'tauri', 'NSIS');

if (!fs.existsSync(src)) {
  console.log('[patch-nsis] skip: custom Korean.nlf not found');
} else if (!fs.existsSync(nsisRoot)) {
  console.log('[patch-nsis] skip: NSIS toolchain not downloaded yet');
} else {
  let patched = 0;
  const walk = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.name.toLowerCase() === 'korean.nlf') {
        const target = fs.readFileSync(full);
        const wanted = fs.readFileSync(src);
        if (!target.equals(wanted)) {
          fs.copyFileSync(src, full);
          patched++;
        }
      }
    }
  };
  walk(nsisRoot);
  console.log('[patch-nsis] Korean.nlf patched: ' + patched);
}
