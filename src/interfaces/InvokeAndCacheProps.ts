interface InvokeAndCacheProps {
  method: string;
  args?: { [key: string]: unknown };
  cacheDuration?: number;
}

export default InvokeAndCacheProps;
