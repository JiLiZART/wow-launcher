import { fetch } from "@tauri-apps/plugin-http";

export type NewsResponse = {
  data: Array<{
    title: string;
    id: number;
    description: string;
    cover_url: string;
    category: {
      name: string;
      id: number;
    };
  }>;
};

export type NewsItem = NewsResponse["data"][0];

export type StatusResponse = {
  realms: Array<{
    id: number;
    name: string;
    isOnline: boolean;
    online: number;
  }>;
  online_count: number;
};

export type StatusRealm = StatusResponse["realms"][0];

export type AddonItem = {
  id: number;
  name: string;
  version: string;
  category: number;
  downloads: number;
  repo_updated_at: string;
  repository: string;
};

export function httpGet<T extends unknown>(url: string): Promise<T> {
  return fetch(url, {
    method: "GET",
    // timeout: 30,
    headers: {
      "User-Agent": `sirus-launcher 1.3.1`,
    },
  }).then((res) => {
    if (res.ok) {
      return res.json();
    }

    return res.text();
  });
}

export function getCoverUrl(url: string) {
  return `https://api.sirus.su${url}`;
}
