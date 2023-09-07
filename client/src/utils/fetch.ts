type RequestConfig = Omit<RequestInit, "body"> & { body: string | number | object | undefined };

export default async function fetch(url: RequestInfo | URL, config?: RequestConfig): Promise<Response> {
  return window.fetch(url, {
    headers: {
      "Content-Type": 'application/json'
    },
    credentials: 'include',
    ...config,
    body: config?.body ? JSON.stringify(config.body) : undefined,
  });
}