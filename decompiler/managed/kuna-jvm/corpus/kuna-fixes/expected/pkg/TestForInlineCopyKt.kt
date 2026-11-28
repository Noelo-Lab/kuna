package pkg

public inline fun <T> InlineFlow<T>.peek(crossinline action: suspend (T) -> Unit): InlineFlow<T> {
   return object : InlineFlow<T> {
      public override suspend fun collect(sink: InlineSink<T>) {
         `$this$peek`.collect(object : InlineSink<T> {
            public override suspend fun emit(value: T) {
               action(value)
               sink.emit(value)
               return Unit.INSTANCE
            }
         })
         return Unit.INSTANCE
      }
   }
}
