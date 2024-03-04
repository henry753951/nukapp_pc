// Source: Henry753951
const fs = require("fs").promises;
const { exec } = require("child_process");
const readline = require("readline").createInterface({
  input: process.stdin,
  output: process.stdout,
});

function updateVersion(version) {
  // Update version in tauri.conf.json
  console.log("📝  更新 tauri.conf.json...");
  const tauriConfPath = "src-tauri/tauri.conf.json";
  
  return fs.readFile(tauriConfPath, "utf8")
    .then((tauriConf) => {
      tauriConf = JSON.parse(tauriConf);
      tauriConf.package.version = version;
      return fs.writeFile(tauriConfPath, JSON.stringify(tauriConf, null, 2));
    })
    .then(() => console.log("📝  tauri.conf.json 更新完成"))
    .catch((error) => console.error("🚫  更新 tauri.conf.json 時發生錯誤：", error));
}

function commitChanges(version) {
  // Stage and commit the change
  console.log("📦  git commit...");
  
  return new Promise((resolve, reject) => {
    exec("git add src-tauri/tauri.conf.json", (error) => {
      if (error) {
        reject(error);
      } else {
        exec(`git commit -m "version ${version} release"`, (error) => {
          if (error) {
            reject(error);
          } else {
            console.log("📦  git commit done");
            resolve();
          }
        });
      }
    });
  });
}

function createTag(version) {
  // Create a new tag
  console.log("🏷️  git tag...");

  return new Promise((resolve, reject) => {
    exec(`git tag v${version}`, (error) => {
      if (error) {
        reject(error);
      } else {
        console.log(`🏷️  git tag v${version} created`);
        resolve();
      }
    });
  });
}

function pushTag(version) {
  // Push the tag to origin
  console.log("🚀  git push...");

  return new Promise((resolve, reject) => {
    exec(`git push origin v${version}`, (error) => {
      if (error) {
        reject(error);
      } else {
        console.log(`🚀  git push origin v${version} done`);
        resolve();
      }
    });
  });
}

let version = "";
readline.question("❤️  請輸入版本號(例如 0.1.0): ", (input) => {
  version = input;
  console.log(`🚀  開始釋出版本 ${version}...`);

  updateVersion(version)
    .then(() => commitChanges(version))
    .then(() => createTag(version))
    .then(() => pushTag(version))
    .then(() => {
      console.log("🎉  版本釋出完成！");
      readline.close();
    })
    .catch((error) => {
      console.error("🚫  版本釋出失敗：", error);
      readline.close();
    });
});