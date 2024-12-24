augroup Cord
    autocmd!
    autocmd VimLeavePre * lua require'cord'.cleanup()
augroup END

function! CordCompleteList(ArgLead, CmdLine, CmdPos)
    let completions = ['build', 'fetch', 'show_presence', 'hide_presence', 'toggle_presence', 'clear_presence', 'idle', 'unidle', 'toggle_idle', 'restart']
    
    return filter(completions, 'v:val =~ "^" . a:ArgLead')
endfunction

command! -nargs=1 -complete=customlist,CordCompleteList Cord lua require'cord.util.usercmd'.handle({<f-args>})