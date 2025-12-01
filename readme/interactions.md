### Representation of interactions 

Our implementation of Interactions is based on the work of
[Mahe et al.](https://doi.org/10.1016/j.scico.2023.103034) and the tool [HIBOU](https://github.com/erwanM974/hibou_label).

We follow the notation of HIBOU for signature files (.hsf)
and interaction files (.hif).

with the signature (sig.hsf):
~~~
@message{
	bwin;cwin;close;blose;busy;msg;sig;free
}

@lifeline{
    l0;l1;l2;l3
}
~~~

and the interaction (i.hif):
~~~
loopS(
	seq(
		par(
			alt(
				l0 -- cwin ->|,
				l0 -- bwin ->|
			),
			busy -> l3 
		),
		msg -> l0,
		sig -> l0,
		l0 -- free -> l3
	)
)
~~~

By using the draw command of HIBOU as follows:

```
hibou_label draw sig.hsf i.hif
```

We obtain the image: 

![i0](images/interactions/i0_repr.png)

#### Gates

 We introduce *gates* in our implementation 
 to mark complementary
communications for the composition.

Gates are assigned by adding number under brackets next 
to the relevant action.

For example, the previous interaction decorated 
with gates is:

~~~
loopS(
	seq(
		par(
			alt(
				l0 -- cwin ->| [3],
				l0 -- bwin ->| [1]
			),
			busy -> l3 [5]
		),
		msg -> l0 [6],
		sig -> l0 [7],
		l0 -- free -> l3
	)
)
~~~

which can be visually represented as:

![i0](images/interactions/i_gates.png)




