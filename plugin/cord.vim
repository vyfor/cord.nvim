augroup Cord
    autocmd!
    autocmd VimLeavePre * lua require'cord'.cleanup()
augroup END