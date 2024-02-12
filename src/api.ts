import { fetch } from "@tauri-apps/api/http";

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

export function httpGet<T>(url: string) {
  return fetch<T>(url, {
    method: "GET",
    timeout: 30,
    headers: {
      "User-Agent": `sirus-launcher 1.3.1`,
    },
  });
}

export function getCoverUrl(url: string) {
  return `https://api.sirus.su${url}`;
}
