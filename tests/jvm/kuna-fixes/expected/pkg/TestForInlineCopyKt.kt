package pkg

public inline fun <T> InlineFlow<T>.peek(crossinline action: suspend (T) -> Unit): InlineFlow<T> {
   return object : InlineFlow<Any> {
      public override suspend fun collect(sink: InlineSink<Any>) {
         `$this$peek`.collect(object : InlineSink<Any> {
            public override suspend fun emit(value: Any) {
               action(value)
               sink.emit(value)
               return Unit.INSTANCE
            }
         })
         return Unit.INSTANCE
      }
   }
}
