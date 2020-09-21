export type AsyncData<T> = {
  status: AsyncStatus;
  data?: T | null;
  error?: Error;
};

export type AsyncStatus = 'idle' | 'loading' | 'error' | 'done' | 'abort';
