# SCWORK
a custom sway workspace switcher (prev n next)

## BUILD
```sh
cargo build --release
```

## USAGE
```swayconf
bindgesture swipe:right exec $HOME/.config/sway/script/scwork prev -min 1 -max 10
bindgesture swipe:left exec $HOME/.config/sway/script/scwork next -min 1 -max 10
```

- EXAMPLE on normal shell
```sh
# for prev
scwork p
scwork prev
# for next
scwork n
scwork next
# using min and max
scwork n -max 5 # will move to min value when workspace bigger than 5
scwork p -min 5 # will move to max value when workspace less than 5
```

## BEHAVIOR
the behavior of this custom prog/script are:
- at ``next`` it will +1 the current workspace name
- at ``prev`` it will -1 the current workspace name
- the default min and max value are 1 and 10
