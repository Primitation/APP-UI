"""Run a TUI in a pseudo-terminal, send keys, check it exits cleanly.
usage: python3 -I tools/pty_smoke.py <cols> <rows> <keys-with-\\n-separators> <binary> [args...]"""
import os, pty, sys, time, select, struct, fcntl, termios, signal
cols, rows, keys = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3].encode().decode('unicode_escape')
cmd = sys.argv[4:]
pid, fd = pty.fork()
if pid == 0:
    os.environ['TERM'] = 'xterm-256color'
    os.execvp(cmd[0], cmd)
fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack('HHHH', rows, cols, 0, 0))
out = b''; t = time.time(); seq = keys.split('|'); sent = 0
while time.time() - t < max(20, 4 + len(seq) * 0.25):
    r, _, _ = select.select([fd], [], [], 0.1)
    if r:
        try: d = os.read(fd, 65536)
        except OSError: break
        if not d: break
        out += d
    if time.time() - t > 1.5 + sent * 0.25 and sent < len(seq):
        os.write(fd, seq[sent].encode()); sent += 1
done, st = os.waitpid(pid, os.WNOHANG)
if done == 0:
    os.kill(pid, signal.SIGKILL); os.waitpid(pid, 0); print('HUNG (killed)'); sys.exit(1)
s = out.decode('utf8', 'replace')
print('exit status', st, '| bytes', len(out), '| panicked' if 'panicked' in s else '| no panic')
for k in sys.argv[5:6]: pass
for word in ['Parts', 'TasksPage', 'WelcomePage', 'Welcome to pond', 'Settings']:
    print(f'  saw {word!r}:', word in s)
