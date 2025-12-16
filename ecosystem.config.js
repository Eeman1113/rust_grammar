module.exports = {
  apps: [{
    name: 'text-analyzer-api',
    script: './target/release/api-server-enhanced',
    // UPDATE THIS PATH TO YOUR ACTUAL DIRECTORY:
    cwd: '/Users/eemanmajumder/code_shit/gramgram/rustam_4/text-analyzer',
    instances: 1,
    autorestart: true,
    watch: false,
    max_memory_restart: '1G',
    env: {
      RUST_LOG: 'info'
    },
    error_file: './logs/err.log',
    out_file: './logs/out.log',
    log_file: './logs/combined.log',
    time: true,
    merge_logs: true,
    log_date_format: 'YYYY-MM-DD HH:mm:ss Z'
  }]
};
