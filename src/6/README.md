
<div class="content-row">

<div class="content-col">

<div class="tab">
  <button class="maintab tablinks active" onclick="switchMainTab(event, 'Source')">Source</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Diff')">Diff</button>
</div>

<div id="Source" class="maintab tabcontent active">

<div class="tab">
<button class="subtab tablinks file-source file-modified active" onclick="switchSubTab(event, 'fundamentals/src/xcm_executor.rs')" data-id="fundamentals/src/xcm_executor.rs">fundamentals/src/xcm_executor.rs</button>
</div>
<div id="source/fundamentals/src/xcm_executor.rs" class="subtab tabcontent active" data-id="fundamentals/src/xcm_executor.rs">

```rust
{{#include ./source/fundamentals/src/xcm_executor.rs}}
```

</div>



</div>

<div id="Diff" class="maintab tabcontent">


<div class="tab">
	<button class="difftab tablinks active" onclick="switchDiff(event, 'changes.diff')" data-id="changes.diff">changes.diff</button>
</div>
<div id="changes.diff" class="difftab tabcontent active" data-id="changes.diff">

```diff
{{#include ./source/changes.diff}}
```

</div>

</div>

</div>
</div>
