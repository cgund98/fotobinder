<script>
    import Entry from "./Entry.svelte";
    import Seperator from "./Seperator.svelte";

    import Search from "../../icons/Search.svelte"
    import Collection from "../../icons/Collection.svelte"
    import Library from "../../icons/Library.svelte"

    const iconClassNames = "stroke-gray-600 h-[20px] w-[20px]"
    const sepClassNames = "px-2"
</script>

<div class="mr-2 bg-bg-gray">
    <div class="flex flex-col">
        <div class="mx-1 mb-2 mt-4">
            <img src="/logo-name.svg" class="w-40" alt="Logo Photobinder" />
        </div>

        <Entry title="Search" href="/search">
            <Search className={iconClassNames}/>
        </Entry>

        <Seperator className={sepClassNames}/>

        <Entry title="Library" href="/library">
            <Library className={iconClassNames} />
        </Entry>

        <Seperator className={sepClassNames}/>

        <Entry title="Collections" href="/collections">
            <Collection className={iconClassNames} />
        </Entry>
    </div>
</div>