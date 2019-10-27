let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd /datadisk/Coding/singularity-rs
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +1 src/credits.rs
badd +0 src/events.rs
badd +81 src/game.rs
badd +23 src/general_unit.rs
badd +6 src/gunit_movement.rs
badd +0 src/layers.rs
badd +1 src/main.rs
badd +0 src/main_v2.rs
badd +0 src/menu.rs
badd +0 src/pause.rs
badd +1 src/platform.rs
badd +0 src/pong.rs
badd +0 src/prefabs.rs
badd +0 src/resources.rs
badd +0 src/roads.rs
badd +13 src/util.rs
badd +0 src/welcome.rs
badd +0 assets/ui/credits.ron
badd +0 assets/ui/custom.ron
badd +0 assets/ui/example.ron
badd +0 assets/ui/fov.ron
badd +0 assets/ui/fps.ron
badd +0 assets/ui/game.ron
badd +0 assets/ui/loading.ron
badd +0 assets/ui/menu.ron
badd +0 assets/ui/menu_v1.ron
badd +0 assets/ui/own.ron
badd +0 assets/ui/pause_menu.ron
badd +0 assets/ui/paused.ron
badd +0 assets/ui/welcome.ron
badd +12 Cargo.toml
argglobal
silent! argdel *
$argadd src/credits.rs
$argadd src/events.rs
$argadd src/game.rs
$argadd src/general_unit.rs
$argadd src/gunit_movement.rs
$argadd src/layers.rs
$argadd src/main.rs
$argadd src/main_v2.rs
$argadd src/menu.rs
$argadd src/pause.rs
$argadd src/platform.rs
$argadd src/pong.rs
$argadd src/prefabs.rs
$argadd src/resources.rs
$argadd src/roads.rs
$argadd src/util.rs
$argadd src/welcome.rs
$argadd assets/ui/credits.ron
$argadd assets/ui/custom.ron
$argadd assets/ui/example.ron
$argadd assets/ui/fov.ron
$argadd assets/ui/fps.ron
$argadd assets/ui/game.ron
$argadd assets/ui/loading.ron
$argadd assets/ui/menu.ron
$argadd assets/ui/menu_v1.ron
$argadd assets/ui/own.ron
$argadd assets/ui/pause_menu.ron
$argadd assets/ui/paused.ron
$argadd assets/ui/welcome.ron
edit src/platform.rs
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd _ | wincmd |
split
1wincmd k
wincmd w
wincmd w
wincmd _ | wincmd |
split
1wincmd k
wincmd w
set nosplitbelow
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 33 + 34) / 69)
exe 'vert 1resize ' . ((&columns * 94 + 94) / 189)
exe '2resize ' . ((&lines * 32 + 34) / 69)
exe 'vert 2resize ' . ((&columns * 94 + 94) / 189)
exe '3resize ' . ((&lines * 33 + 34) / 69)
exe 'vert 3resize ' . ((&columns * 94 + 94) / 189)
exe '4resize ' . ((&lines * 32 + 34) / 69)
exe 'vert 4resize ' . ((&columns * 94 + 94) / 189)
argglobal
if bufexists('src/platform.rs') | buffer src/platform.rs | else | edit src/platform.rs | endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 99 - ((29 * winheight(0) + 16) / 33)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
99
normal! 0
wincmd w
argglobal
if bufexists('src/game.rs') | buffer src/game.rs | else | edit src/game.rs | endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 136 - ((30 * winheight(0) + 16) / 32)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
136
normal! 050|
wincmd w
argglobal
if bufexists('src/general_unit.rs') | buffer src/general_unit.rs | else | edit src/general_unit.rs | endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 32 - ((31 * winheight(0) + 16) / 33)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
32
normal! 08|
wincmd w
argglobal
if bufexists('src/gunit_movement.rs') | buffer src/gunit_movement.rs | else | edit src/gunit_movement.rs | endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 24 - ((23 * winheight(0) + 16) / 32)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
24
normal! 041|
wincmd w
4wincmd w
exe '1resize ' . ((&lines * 33 + 34) / 69)
exe 'vert 1resize ' . ((&columns * 94 + 94) / 189)
exe '2resize ' . ((&lines * 32 + 34) / 69)
exe 'vert 2resize ' . ((&columns * 94 + 94) / 189)
exe '3resize ' . ((&lines * 33 + 34) / 69)
exe 'vert 3resize ' . ((&columns * 94 + 94) / 189)
exe '4resize ' . ((&lines * 32 + 34) / 69)
exe 'vert 4resize ' . ((&columns * 94 + 94) / 189)
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOF
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
