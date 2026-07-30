int4 main(int4 argc,char **argv)

{
  server *srv; // rax
  uint4 v1;
  int4 v2; // eax
  int8 v3; // rax
  
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if (getuid() != 0) {
    v2 = geteuid();
    if (v2 == getuid()) {
      v2 = getegid();
      if (v2 == getgid()) goto label_f6ec;
    }
    v2 = -1;
    fputs("Are you nuts ? Don\'t apply a SUID bit to this binary\n",stderr);
  }
  else {
label_f6ec:
    setlocale(2,"C");
    tzset();
    do {
      srv = calloc(1,0x1c8);
      if (srv == (server *)0x0) {
                    /* WARNING: Subroutine does not return */
        log_failed_assert("server.c",0xe7,"assertion failed: srv");
      }
      *(void *)((int8)srv + 0x28) = buffer_init();
      connection_joblist = (void *)((int8)srv + 0x40);
      strftime_cache_reset();
      li_rand_reseed();
      log_epoch_secs = time(0);
      *(void *)((int8)srv + 0x1a0) = log_epoch_secs;
      *(log_error_st **)((int8)srv + 0x98) = log_error_st_init();
      config_init(srv);
      *(void *)((int8)srv + 0xa8) = 0;
      *(void *)((int8)srv + 0xb0) = 0;
      *(code **)((int8)srv + 0x20) = plugins_call_handle_request_env;
      *(void *)((int8)srv + 0xb8) = 0;
      *(void *)((int8)srv + 0x1bc) = 0xffffffff;
      if (dat_5f278 != 0) {
        dat_5f290 = 0;
        dat_5f298 = 0;
        *(void *)((int8)srv + 0x170) = dat_5f290;
        *(void *)((int8)srv + 0x178) = dat_5f298;
        v1 = *(uint4 *)((int8)srv + 0x17c);
        *(void *)((int8)srv + 0x180) = dat_5f280;
        *(void *)((int8)srv + 0x188) = dat_5f288;
        dat_5f280 = 0;
        dat_5f288 = 0;
        for (v3 = 0; (uint4)v3 < v1; v3 = v3 + 1) {
          *(server **)(*(int8 *)(*(int8 *)((int8)srv + 0x170) + v3 * 8) + 0x80) = srv;
        }
        v1 = *(uint4 *)((int8)srv + 0x18c);
        for (v3 = 0; (uint4)v3 < v1; v3 = v3 + 1) {
          *(server **)(*(int8 *)(*(int8 *)((int8)srv + 0x180) + v3 * 8) + 0x80) = srv;
        }
        optind = 1;
      }
      v2 = server_main_setup(srv,argc,argv);
      if (1 <= v2) {
        server_main_loop(srv);
        if ((dat_5f274 != 0) || (dat_5f278 != 0)) {
          server_graceful_state(srv);
        }
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
        if (dat_5f274 != 2) {
          if (oneshot_fd == 0) {
            log_error(*(void **)((int8)srv + 0x98),"server.c",0x7b8,"server stopped by UID = %d PID = %d",(uint8)dat_5f1f4,(uint8)dat_5f1f0);
          }
        }
        else {
          log_error(*(void **)((int8)srv + 0x98),"server.c",0x7b4,"server stopped after idle timeout");
        }
      }
      remove_pid_file(srv);
      config_log_error_close(srv);
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
      if (dat_5f278 != 0) {
        v1 = *(uint4 *)((int8)srv + 0x17c);
        for (v3 = 0; (uint4)v3 < v1; v3 = v3 + 1) {
          *(void *)(*(int8 *)(*(int8 *)((int8)srv + 0x170) + v3 * 8) + 0x80) = 0;
        }
        v1 = *(uint4 *)((int8)srv + 0x18c);
        for (v3 = 0; (uint4)v3 < v1; v3 = v3 + 1) {
          *(void *)(*(int8 *)(*(int8 *)((int8)srv + 0x180) + v3 * 8) + 0x80) = 0;
        }
        dat_5f290 = *(void *)((int8)srv + 0x170);
        dat_5f298 = *(void *)((int8)srv + 0x178);
        dat_5f280 = *(void *)((int8)srv + 0x180);
        dat_5f288 = *(void *)((int8)srv + 0x188);
        *(void *)((int8)srv + 0x170) = 0;
        *(void *)((int8)srv + 0x178) = 0;
        *(void *)((int8)srv + 0x180) = 0;
        *(void *)((int8)srv + 0x188) = 0;
      }
      else {
        network_close(srv);
      }
      request_pool_free();
      connections_free(srv);
      plugins_free(srv);
      if (1 <= oneshot_fd) {
        if (oneshot_fdn != 0) {
          fdevent_fdnode_event_del(*(void *)((int8)srv + 0x10));
          fdevent_unregister(*(fdevents **)((int8)srv + 0x10),oneshot_fd);
          oneshot_fdn = 0;
        }
        close(oneshot_fd);
      }
      if (0 <= oneshot_fdout) {
        close();
      }
      if (0 <= *(int4 *)((int8)srv + 0x1bc)) {
        close();
      }
      buffer_free(*(anon_struct **)((int8)srv + 0x28));
      fdevent_free(*(fdevents **)((int8)srv + 0x10));
      config_free(srv);
      free(*(void **)((int8)srv + 0x40));
      free(*(void **)((int8)srv + 0x50));
      free(*(void **)((int8)srv + 0x60));
      stat_cache_free();
      li_rand_cleanup();
      chunkqueue_chunk_pool_free();
      log_error_st_free(*(log_error_st **)((int8)srv + 0x98));
      free(srv);
      if (v2 < 0) {
        return v2;
      }
      if (dat_5f278 == 0) {
        return v2;
      }
      do {
      } while (0 < fdevent_waitpid(-1,(int4 *)0x0,0));
    } while (dat_5f278 != 0);
  }
  return v2;
}
