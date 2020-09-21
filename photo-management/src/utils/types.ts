export type AsyncData<T> =
  | {
      status: AsyncStatus;
      data: null;
    }
  | {
      status: 'done';
      data: T;
    }
  | {
      status: 'error';
      error: Error;
    };

export type AsyncStatus = 'idle' | 'loading' | 'abort';
export type RequestStatus = AsyncData<{}>['status'];
