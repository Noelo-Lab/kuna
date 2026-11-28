package pkg

public suspend fun forward(f: SamIntFlow, out: SamIntSink) {
   f.collect(object : SamIntSink {
      override final suspend fun emit(v: Int) {
         out.emit(v)
         out.emit(v * 2)
         return Unit.INSTANCE
      }
   })
   return Unit.INSTANCE
}

public suspend fun relay(f: SamIntFlow, out: SamIntSink) {
   f.collect(object : SamIntSink {
      override final suspend fun emit(v: Int) {
         out.emit(v + 1)
         return Unit.INSTANCE
      }
   })
   return Unit.INSTANCE
}

public suspend fun lengths(f: SamFlow<String>, out: SamSink<Int>) {
   f.collect(object : SamSink {
      override final suspend fun emit(s: String) {
         if ((s as CharSequence).length > 0) {
            out.emit(s.length)
         }

         out.emit(-1)
         return Unit.INSTANCE
      }
   })
   return Unit.INSTANCE
}
