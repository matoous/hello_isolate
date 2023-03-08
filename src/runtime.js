globalThis.workerHandler = (x) => {
    if (typeof handler !== 'function') {
      throw new Error('Handler function is not defined or is not a function');
    }    
    return handler(x)
}
