# SCWORK
a custom sway workspace switcher (prev n next)

## BUILD
```sh
cargo build --release
```

## USAGE
```swayconf
bindgesture swipe:right exec $HOME/.config/sway/script/scwork prev
bindgesture swipe:left exec $HOME/.config/sway/script/scwork next 
```

- EXAMPLE on normal shell
```sh
# for prev
scwork p
scwork prev
# for next
scwork n
scwork next
```

## LIMITATION
- for now it only support 10 workspaces
- when using prev on workspace 1 and next on workspace 10 will do nothing
- only support workspaces with 1-10 name

## BEHAVIOR
the behavior of this custom prog/script are:
- at ``next`` it will +1 the current workspace name
- at ``prev`` it will -1 the current workspace name
