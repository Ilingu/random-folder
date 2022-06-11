import { FileEntry, readBinaryFile, readDir } from "@tauri-apps/api/fs";
import { open as OpenFolderPicker } from "@tauri-apps/api/dialog";
import { Command } from "@tauri-apps/api/shell";
import { FunctionJobs } from "./interfaces/interfaces";

export const ReadDir = async (
  path: string
): Promise<FunctionJobs<FileEntry[]>> => {
  try {
    const FilesRes = await readDir(path);
    return { success: true, returns: FilesRes };
  } catch (err) {
    return { success: false };
  }
};

export const OpenDialog = async (): Promise<FunctionJobs<string>> => {
  try {
    const FolderSelected = await OpenFolderPicker({
      directory: true,
      defaultPath: ".",
    });
    if (!FolderSelected) return { success: false };
    return { success: true, returns: FolderSelected.toString() };
  } catch (err) {
    return { success: false };
  }
};

export const OpenFolder = async (path: string) => {
  await new Command("explorer", path).execute();
};

export const ReadImage = async (
  FolderPath: string
): Promise<FunctionJobs<string>> => {
  return new Promise(async (res) => {
    const Attempts = [
      "01.jpg",
      "001.jpg",
      "01.png",
      "001.png",
      "1.jpg",
      "1.png",
    ];

    for (const attempt of Attempts) {
      try {
        // const startedAt = Date.now();
        // console.log(`file read in ${Date.now() - startedAt}ms`);

        const ImageBinary = await readBinaryFile(`${FolderPath}\\${attempt}`); // Fetching Image
        if (ImageBinary.length <= 0) continue;
        console.warn(`Image found for "${attempt}"`);

        // let Binary = "";
        // for (const ImageByte of ImageBinary) {
        //   Binary += String.fromCharCode(ImageByte); // Decoding Bytes by Bytes
        // }
        // const ImageB64 = btoa(Binary); // To Base64

        const ImageBlob = new Blob([ImageBinary.buffer], {
          type: `image/${attempt.split(".")[1]}`,
        });
        const ImageURL = URL.createObjectURL(ImageBlob);

        return res({ success: true, returns: ImageURL });
      } catch (err) {
        console.warn(`Cannot Find Image for "${attempt}", trying next...`);
        continue;
      }
    }

    return res({ success: false });
  });
};
