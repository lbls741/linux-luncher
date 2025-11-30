import { invoke } from '@tauri-apps/api/core';

/**
 * Downloads a file from a URL to a specified file path.
 *
 * @param url The URL of the file to download.
 * @param filePath The name of the file to save in the downloads directory.
 * @returns A promise that resolves when the download is complete.
 */
export async function downloadFile(url: string, filePath: string): Promise<void> {
  try {
    await invoke('download_file', { url, filePath });
    console.log(`File downloaded successfully to downloads/${filePath}`);
  } catch (error) {
    console.error('Download failed:', error);
    throw error;
  }
}
