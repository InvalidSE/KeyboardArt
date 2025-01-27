<script lang="ts">
    import { onMount } from 'svelte';

    // 107 long array of RGB colours (i know this is bad code)
    let keyColours = ["#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666","#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666","#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666", "#666666"];
    let mousedown = false;

    // function to convert RGB to Hex
    function rgbToHex(r: number, g: number, b: number): string {
        return "#" + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
    }

    function handlePaint(key: number) {;
        keyColours[key] = "#E5E7EB";
        // console.log(key);
    }

    onMount(() => {
        window.addEventListener("mousedown", () => {
            mousedown = true;
        });
        window.addEventListener("mouseup", () => {
            mousedown = false;
        });
    });
    

</script>

<div class="w-screen h-screen place-content-center text-center">
    <!-- <span class="text-5xl md:text-7xl lg:text-6xl font-bold primary">
    </span> -->

    <svg class="mx-auto" width="954" height="258" viewBox="0 0 954 258" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect width="35" height="35" rx="4" fill={keyColours[0]} id="0" role="button" tabindex=0 on:click={() => handlePaint(0)} on:mouseenter={() => mousedown === true && handlePaint(0)} on:keydown={(e) => e.key === 'Enter' && handlePaint(0)}/>
        <rect y="51" width="35" height="35" rx="4" fill={keyColours[1]} role="button" tabindex=0 on:click={() => handlePaint(1)} on:mouseenter={() => mousedown === true && handlePaint(1)} on:keydown={(e) => e.key === 'Enter' && handlePaint(1)}/>
        <rect y="94" width="56" height="35" rx="4" fill={keyColours[2]} role="button" tabindex=0 on:click={() => handlePaint(2)} on:mouseenter={() => mousedown === true && handlePaint(2)} on:keydown={(e) => e.key === 'Enter' && handlePaint(2)}/>
        <rect y="137" width="64" height="35" rx="4" fill={keyColours[3]} role="button" tabindex=0 on:click={() => handlePaint(3)} on:mouseenter={() => mousedown === true && handlePaint(3)} on:keydown={(e) => e.key === 'Enter' && handlePaint(3)}/>
        <rect y="180" width="86" height="35" rx="4" fill={keyColours[4]} role="button" tabindex=0 on:click={() => handlePaint(4)} on:mouseenter={() => mousedown === true && handlePaint(4)} on:keydown={(e) => e.key === 'Enter' && handlePaint(4)}/>
        <rect y="223" width="45" height="35" rx="4" fill={keyColours[5]} role="button" tabindex=0 on:click={() => handlePaint(5)} on:mouseenter={() => mousedown === true && handlePaint(5)} on:keydown={(e) => e.key === 'Enter' && handlePaint(5)}/>
        <rect x="43" y="51" width="35" height="35" rx="4" fill={keyColours[7]} role="button" tabindex=0 on:click={() => handlePaint(7)} on:mouseenter={() => mousedown === true && handlePaint(7)} on:keydown={(e) => e.key === 'Enter' && handlePaint(7)}/>
        <rect x="64" y="94" width="35" height="35" rx="4" fill={keyColours[8]} role="button" tabindex=0 on:click={() => handlePaint(8)} on:mouseenter={() => mousedown === true && handlePaint(8)} on:keydown={(e) => e.key === 'Enter' && handlePaint(8)}/>
        <rect x="72" y="137" width="35" height="35" rx="4" fill={keyColours[9]} role="button" tabindex=0 on:click={() => handlePaint(9)} on:mouseenter={() => mousedown === true && handlePaint(9)} on:keydown={(e) => e.key === 'Enter' && handlePaint(9)}/>
        <rect x="94" y="180" width="35" height="35" rx="4" fill={keyColours[10]} role="button" tabindex=0 on:click={() => handlePaint(10)} on:mouseenter={() => mousedown === true && handlePaint(10)} on:keydown={(e) => e.key === 'Enter' && handlePaint(10)}/>
        <rect x="53" y="223" width="45" height="35" rx="4" fill={keyColours[11]} role="button" tabindex=0 on:click={() => handlePaint(11)} on:mouseenter={() => mousedown === true && handlePaint(11)} on:keydown={(e) => e.key === 'Enter' && handlePaint(11)}/>
        <rect x="86" width="35" height="35" rx="4" fill={keyColours[12]} role="button" tabindex=0 on:click={() => handlePaint(12)} on:mouseenter={() => mousedown === true && handlePaint(12)} on:keydown={(e) => e.key === 'Enter' && handlePaint(12)}/>
        <rect x="86" y="51" width="35" height="35" rx="4" fill={keyColours[13]} role="button" tabindex=0 on:click={() => handlePaint(13)} on:mouseenter={() => mousedown === true && handlePaint(13)} on:keydown={(e) => e.key === 'Enter' && handlePaint(13)}/>
        <rect x="107" y="94" width="35" height="35" rx="4" fill={keyColours[14]} role="button" tabindex=0 on:click={() => handlePaint(14)} on:mouseenter={() => mousedown === true && handlePaint(14)} on:keydown={(e) => e.key === 'Enter' && handlePaint(14)}/>
        <rect x="115" y="137" width="35" height="35" rx="4" fill={keyColours[15]} role="button" tabindex=0 on:click={() => handlePaint(15)} on:mouseenter={() => mousedown === true && handlePaint(15)} on:keydown={(e) => e.key === 'Enter' && handlePaint(15)}/>
        <rect x="137" y="180" width="35" height="35" rx="4" fill={keyColours[16]} role="button" tabindex=0 on:click={() => handlePaint(16)} on:mouseenter={() => mousedown === true && handlePaint(16)} on:keydown={(e) => e.key === 'Enter' && handlePaint(16)}/>
        <rect x="106" y="223" width="45" height="35" rx="4" fill={keyColours[17]} role="button" tabindex=0 on:click={() => handlePaint(17)} on:mouseenter={() => mousedown === true && handlePaint(17)} on:keydown={(e) => e.key === 'Enter' && handlePaint(17)}/>
        <rect x="129" width="35" height="35" rx="4" fill={keyColours[18]} role="button" tabindex=0 on:click={() => handlePaint(18)} on:mouseenter={() => mousedown === true && handlePaint(18)} on:keydown={(e) => e.key === 'Enter' && handlePaint(18)}/>
        <rect x="129" y="51" width="35" height="35" rx="4" fill={keyColours[19]} role="button" tabindex=0 on:click={() => handlePaint(19)} on:mouseenter={() => mousedown === true && handlePaint(19)} on:keydown={(e) => e.key === 'Enter' && handlePaint(19)}/>
        <rect x="150" y="94" width="35" height="35" rx="4" fill={keyColours[20]} role="button" tabindex=0 on:click={() => handlePaint(20)} on:mouseenter={() => mousedown === true && handlePaint(20)} on:keydown={(e) => e.key === 'Enter' && handlePaint(20)}/>
        <rect x="158" y="137" width="35" height="35" rx="4" fill={keyColours[21]} role="button" tabindex=0 on:click={() => handlePaint(21)} on:mouseenter={() => mousedown === true && handlePaint(21)} on:keydown={(e) => e.key === 'Enter' && handlePaint(21)}/>
        <rect x="180" y="180" width="35" height="35" rx="4" fill={keyColours[22]} role="button" tabindex=0 on:click={() => handlePaint(22)} on:mouseenter={() => mousedown === true && handlePaint(22)} on:keydown={(e) => e.key === 'Enter' && handlePaint(22)}/>
        <rect x="172" width="35" height="35" rx="4" fill={keyColours[23]} role="button" tabindex=0 on:click={() => handlePaint(23)} on:mouseenter={() => mousedown === true && handlePaint(23)} on:keydown={(e) => e.key === 'Enter' && handlePaint(23)}/>
        <rect x="172" y="51" width="35" height="35" rx="4" fill={keyColours[24]} role="button" tabindex=0 on:click={() => handlePaint(24)} on:mouseenter={() => mousedown === true && handlePaint(24)} on:keydown={(e) => e.key === 'Enter' && handlePaint(24)}/>
        <rect x="193" y="94" width="35" height="35" rx="4" fill={keyColours[25]} role="button" tabindex=0 on:click={() => handlePaint(25)} on:mouseenter={() => mousedown === true && handlePaint(25)} on:keydown={(e) => e.key === 'Enter' && handlePaint(25)}/>
        <rect x="201" y="137" width="35" height="35" rx="4" fill={keyColours[26]} role="button" tabindex=0 on:click={() => handlePaint(26)} on:mouseenter={() => mousedown === true && handlePaint(26)} on:keydown={(e) => e.key === 'Enter' && handlePaint(26)}/>
        <rect x="223" y="180" width="35" height="35" rx="4" fill={keyColours[27]} role="button" tabindex=0 on:click={() => handlePaint(27)} on:mouseenter={() => mousedown === true && handlePaint(27)} on:keydown={(e) => e.key === 'Enter' && handlePaint(27)}/>
        <rect x="215" width="35" height="35" rx="4" fill={keyColours[28]} role="button" tabindex=0 on:click={() => handlePaint(28)} on:mouseenter={() => mousedown === true && handlePaint(28)} on:keydown={(e) => e.key === 'Enter' && handlePaint(28)}/>
        <rect x="215" y="51" width="35" height="35" rx="4" fill={keyColours[29]} role="button" tabindex=0 on:click={() => handlePaint(29)} on:mouseenter={() => mousedown === true && handlePaint(29)} on:keydown={(e) => e.key === 'Enter' && handlePaint(29)}/>
        <rect x="236" y="94" width="35" height="35" rx="4" fill={keyColours[30]} role="button" tabindex=0 on:click={() => handlePaint(30)} on:mouseenter={() => mousedown === true && handlePaint(30)} on:keydown={(e) => e.key === 'Enter' && handlePaint(30)}/>
        <rect x="244" y="137" width="35" height="35" rx="4" fill={keyColours[31]} role="button" tabindex=0 on:click={() => handlePaint(31)} on:mouseenter={() => mousedown === true && handlePaint(31)} on:keydown={(e) => e.key === 'Enter' && handlePaint(31)}/>
        <rect x="266" y="180" width="35" height="35" rx="4" fill={keyColours[32]} role="button" tabindex=0 on:click={() => handlePaint(32)} on:mouseenter={() => mousedown === true && handlePaint(32)} on:keydown={(e) => e.key === 'Enter' && handlePaint(32)}/>
        <rect x="159" y="223" width="266" height="35" rx="4" fill={keyColours[33]} role="button" tabindex=0 on:click={() => handlePaint(33)} on:mouseenter={() => mousedown === true && handlePaint(33)} on:keydown={(e) => e.key === 'Enter' && handlePaint(33)}/>
        <rect x="279" width="35" height="35" rx="4" fill={keyColours[34]} role="button" tabindex=0 on:click={() => handlePaint(34)} on:mouseenter={() => mousedown === true && handlePaint(34)} on:keydown={(e) => e.key === 'Enter' && handlePaint(34)}/>
        <rect x="258" y="51" width="35" height="35" rx="4" fill={keyColours[35]} role="button" tabindex=0 on:click={() => handlePaint(35)} on:mouseenter={() => mousedown === true && handlePaint(35)} on:keydown={(e) => e.key === 'Enter' && handlePaint(35)}/>
        <rect x="279" y="94" width="35" height="35" rx="4" fill={keyColours[36]} role="button" tabindex=0 on:click={() => handlePaint(36)} on:mouseenter={() => mousedown === true && handlePaint(36)} on:keydown={(e) => e.key === 'Enter' && handlePaint(36)}/>
        <rect x="287" y="137" width="35" height="35" rx="4" fill={keyColours[37]} role="button" tabindex=0 on:click={() => handlePaint(37)} on:mouseenter={() => mousedown === true && handlePaint(37)} on:keydown={(e) => e.key === 'Enter' && handlePaint(37)}/>
        <rect x="309" y="180" width="35" height="35" rx="4" fill={keyColours[38]} role="button" tabindex=0 on:click={() => handlePaint(38)} on:mouseenter={() => mousedown === true && handlePaint(38)} on:keydown={(e) => e.key === 'Enter' && handlePaint(38)}/>
        <rect x="322" width="35" height="35" rx="4" fill={keyColours[39]} role="button" tabindex=0 on:click={() => handlePaint(39)} on:mouseenter={() => mousedown === true && handlePaint(39)} on:keydown={(e) => e.key === 'Enter' && handlePaint(39)}/>
        <rect x="301" y="51" width="35" height="35" rx="4" fill={keyColours[40]} role="button" tabindex=0 on:click={() => handlePaint(40)} on:mouseenter={() => mousedown === true && handlePaint(40)} on:keydown={(e) => e.key === 'Enter' && handlePaint(40)}/>
        <rect x="322" y="94" width="35" height="35" rx="4" fill={keyColours[41]} role="button" tabindex=0 on:click={() => handlePaint(41)} on:mouseenter={() => mousedown === true && handlePaint(41)} on:keydown={(e) => e.key === 'Enter' && handlePaint(41)}/>
        <rect x="330" y="137" width="35" height="35" rx="4" fill={keyColours[42]} role="button" tabindex=0 on:click={() => handlePaint(42)} on:mouseenter={() => mousedown === true && handlePaint(42)} on:keydown={(e) => e.key === 'Enter' && handlePaint(42)}/>
        <rect x="352" y="180" width="35" height="35" rx="4" fill={keyColours[43]} role="button" tabindex=0 on:click={() => handlePaint(43)} on:mouseenter={() => mousedown === true && handlePaint(43)} on:keydown={(e) => e.key === 'Enter' && handlePaint(43)}/>
        <rect x="365" width="35" height="35" rx="4" fill={keyColours[44]} role="button" tabindex=0 on:click={() => handlePaint(44)} on:mouseenter={() => mousedown === true && handlePaint(44)} on:keydown={(e) => e.key === 'Enter' && handlePaint(44)}/>
        <rect x="344" y="51" width="35" height="35" rx="4" fill={keyColours[45]} role="button" tabindex=0 on:click={() => handlePaint(45)} on:mouseenter={() => mousedown === true && handlePaint(45)} on:keydown={(e) => e.key === 'Enter' && handlePaint(45)}/>
        <rect x="365" y="94" width="35" height="35" rx="4" fill={keyColours[46]} role="button" tabindex=0 on:click={() => handlePaint(46)} on:mouseenter={() => mousedown === true && handlePaint(46)} on:keydown={(e) => e.key === 'Enter' && handlePaint(46)}/>
        <rect x="373" y="137" width="35" height="35" rx="4" fill={keyColours[47]} role="button" tabindex=0 on:click={() => handlePaint(47)} on:mouseenter={() => mousedown === true && handlePaint(47)} on:keydown={(e) => e.key === 'Enter' && handlePaint(47)}/>
        <rect x="395" y="180" width="35" height="35" rx="4" fill={keyColours[48]} role="button" tabindex=0 on:click={() => handlePaint(48)} on:mouseenter={() => mousedown === true && handlePaint(48)} on:keydown={(e) => e.key === 'Enter' && handlePaint(48)}/>
        <rect x="433" y="223" width="45" height="35" rx="4" fill={keyColours[49]} role="button" tabindex=0 on:click={() => handlePaint(49)} on:mouseenter={() => mousedown === true && handlePaint(49)} on:keydown={(e) => e.key === 'Enter' && handlePaint(49)}/>
        <rect x="408" width="35" height="35" rx="4" fill={keyColours[50]} role="button" tabindex=0 on:click={() => handlePaint(50)} on:mouseenter={() => mousedown === true && handlePaint(50)} on:keydown={(e) => e.key === 'Enter' && handlePaint(50)}/>
        <rect x="387" y="51" width="35" height="35" rx="4" fill={keyColours[51]} role="button" tabindex=0 on:click={() => handlePaint(51)} on:mouseenter={() => mousedown === true && handlePaint(51)} on:keydown={(e) => e.key === 'Enter' && handlePaint(51)}/>
        <rect x="408" y="94" width="35" height="35" rx="4" fill={keyColours[52]} role="button" tabindex=0 on:click={() => handlePaint(52)} on:mouseenter={() => mousedown === true && handlePaint(52)} on:keydown={(e) => e.key === 'Enter' && handlePaint(52)}/>
        <rect x="416" y="137" width="35" height="35" rx="4" fill={keyColours[53]} role="button" tabindex=0 on:click={() => handlePaint(53)} on:mouseenter={() => mousedown === true && handlePaint(53)} on:keydown={(e) => e.key === 'Enter' && handlePaint(53)}/>
        <rect x="438" y="180" width="35" height="35" rx="4" fill={keyColours[54]} role="button" tabindex=0 on:click={() => handlePaint(54)} on:mouseenter={() => mousedown === true && handlePaint(54)} on:keydown={(e) => e.key === 'Enter' && handlePaint(54)}/>
        <rect x="473" width="35" height="35" rx="4" fill={keyColours[55]} role="button" tabindex=0 on:click={() => handlePaint(55)} on:mouseenter={() => mousedown === true && handlePaint(55)} on:keydown={(e) => e.key === 'Enter' && handlePaint(55)}/>
        <rect x="430" y="51" width="35" height="35" rx="4" fill={keyColours[56]} role="button" tabindex=0 on:click={() => handlePaint(56)} on:mouseenter={() => mousedown === true && handlePaint(56)} on:keydown={(e) => e.key === 'Enter' && handlePaint(56)}/>
        <rect x="451" y="94" width="35" height="35" rx="4" fill={keyColours[57]} role="button" tabindex=0 on:click={() => handlePaint(57)} on:mouseenter={() => mousedown === true && handlePaint(57)} on:keydown={(e) => e.key === 'Enter' && handlePaint(57)}/>
        <rect x="459" y="137" width="35" height="35" rx="4" fill={keyColours[58]} role="button" tabindex=0 on:click={() => handlePaint(58)} on:mouseenter={() => mousedown === true && handlePaint(58)} on:keydown={(e) => e.key === 'Enter' && handlePaint(58)}/>
        <rect x="481" y="180" width="35" height="35" rx="4" fill={keyColours[59]} role="button" tabindex=0 on:click={() => handlePaint(59)} on:mouseenter={() => mousedown === true && handlePaint(59)} on:keydown={(e) => e.key === 'Enter' && handlePaint(59)}/>
        <rect x="486" y="223" width="45" height="35" rx="4" fill={keyColours[60]} role="button" tabindex=0 on:click={() => handlePaint(60)} on:mouseenter={() => mousedown === true && handlePaint(60)} on:keydown={(e) => e.key === 'Enter' && handlePaint(60)}/>
        <rect x="516" width="35" height="35" rx="4" fill={keyColours[61]} role="button" tabindex=0 on:click={() => handlePaint(61)} on:mouseenter={() => mousedown === true && handlePaint(61)} on:keydown={(e) => e.key === 'Enter' && handlePaint(61)}/>
        <rect x="473" y="51" width="35" height="35" rx="4" fill={keyColours[62]} role="button" tabindex=0 on:click={() => handlePaint(62)} on:mouseenter={() => mousedown === true && handlePaint(62)} on:keydown={(e) => e.key === 'Enter' && handlePaint(62)}/>
        <rect x="494" y="94" width="35" height="35" rx="4" fill={keyColours[63]} role="button" tabindex=0 on:click={() => handlePaint(63)} on:mouseenter={() => mousedown === true && handlePaint(63)} on:keydown={(e) => e.key === 'Enter' && handlePaint(63)}/>
        <rect x="502" y="137" width="35" height="35" rx="4" fill={keyColours[64]} role="button" tabindex=0 on:click={() => handlePaint(64)} on:mouseenter={() => mousedown === true && handlePaint(64)} on:keydown={(e) => e.key === 'Enter' && handlePaint(64)}/>
        <rect x="559" width="35" height="35" rx="4" fill={keyColours[65]} role="button" tabindex=0 on:click={() => handlePaint(65)} on:mouseenter={() => mousedown === true && handlePaint(65)} on:keydown={(e) => e.key === 'Enter' && handlePaint(65)}/>
        <rect x="516" y="51" width="35" height="35" rx="4" fill={keyColours[66]} role="button" tabindex=0 on:click={() => handlePaint(66)} on:mouseenter={() => mousedown === true && handlePaint(66)} on:keydown={(e) => e.key === 'Enter' && handlePaint(66)}/>
        <rect x="537" y="94" width="35" height="35" rx="4" fill={keyColours[67]} role="button" tabindex=0 on:click={() => handlePaint(67)} on:mouseenter={() => mousedown === true && handlePaint(67)} on:keydown={(e) => e.key === 'Enter' && handlePaint(67)}/>
        <rect x="539" y="223" width="45" height="35" rx="4" fill={keyColours[70]} role="button" tabindex=0 on:click={() => handlePaint(70)} on:mouseenter={() => mousedown === true && handlePaint(70)} on:keydown={(e) => e.key === 'Enter' && handlePaint(70)}/>
        <rect x="602" width="35" height="35" rx="4" fill={keyColours[71]} role="button" tabindex=0 on:click={() => handlePaint(71)} on:mouseenter={() => mousedown === true && handlePaint(71)} on:keydown={(e) => e.key === 'Enter' && handlePaint(71)}/>
        <rect x="559" y="51" width="78" height="35" rx="4" fill={keyColours[72]} role="button" tabindex=0 on:click={() => handlePaint(72)} on:mouseenter={() => mousedown === true && handlePaint(72)} on:keydown={(e) => e.key === 'Enter' && handlePaint(72)}/>
        <rect x="580" y="94" width="57" height="35" rx="4" fill={keyColours[73]} role="button" tabindex=0 on:click={() => handlePaint(73)} on:mouseenter={() => mousedown === true && handlePaint(73)} on:keydown={(e) => e.key === 'Enter' && handlePaint(73)}/>
        <rect x="545" y="137" width="92" height="35" rx="4" fill={keyColours[74]} role="button" tabindex=0 on:click={() => handlePaint(74)} on:mouseenter={() => mousedown === true && handlePaint(74)} on:keydown={(e) => e.key === 'Enter' && handlePaint(74)}/>
        <rect x="524" y="180" width="113" height="35" rx="4" fill={keyColours[75]} role="button" tabindex=0 on:click={() => handlePaint(75)} on:mouseenter={() => mousedown === true && handlePaint(75)} on:keydown={(e) => e.key === 'Enter' && handlePaint(75)}/>
        <rect x="592" y="223" width="45" height="35" rx="4" fill={keyColours[76]} role="button" tabindex=0 on:click={() => handlePaint(76)} on:mouseenter={() => mousedown === true && handlePaint(76)} on:keydown={(e) => e.key === 'Enter' && handlePaint(76)}/>
        <rect x="653" width="35" height="35" rx="4" fill={keyColours[77]} role="button" tabindex=0 on:click={() => handlePaint(77)} on:mouseenter={() => mousedown === true && handlePaint(77)} on:keydown={(e) => e.key === 'Enter' && handlePaint(77)}/>
        <rect x="653" y="51" width="35" height="35" rx="4" fill={keyColours[78]} role="button" tabindex=0 on:click={() => handlePaint(78)} on:mouseenter={() => mousedown === true && handlePaint(78)} on:keydown={(e) => e.key === 'Enter' && handlePaint(78)}/>
        <rect x="653" y="94" width="35" height="35" rx="4" fill={keyColours[79]} role="button" tabindex=0 on:click={() => handlePaint(79)} on:mouseenter={() => mousedown === true && handlePaint(79)} on:keydown={(e) => e.key === 'Enter' && handlePaint(79)}/>
        <rect x="653" y="223" width="35" height="35" rx="4" fill={keyColours[80]} role="button" tabindex=0 on:click={() => handlePaint(80)} on:mouseenter={() => mousedown === true && handlePaint(80)} on:keydown={(e) => e.key === 'Enter' && handlePaint(80)}/>
        <rect x="696" width="35" height="35" rx="4" fill={keyColours[81]} role="button" tabindex=0 on:click={() => handlePaint(81)} on:mouseenter={() => mousedown === true && handlePaint(81)} on:keydown={(e) => e.key === 'Enter' && handlePaint(81)}/>
        <rect x="696" y="51" width="35" height="35" rx="4" fill={keyColours[82]} role="button" tabindex=0 on:click={() => handlePaint(82)} on:mouseenter={() => mousedown === true && handlePaint(82)} on:keydown={(e) => e.key === 'Enter' && handlePaint(82)}/>
        <rect x="696" y="94" width="35" height="35" rx="4" fill={keyColours[83]} role="button" tabindex=0 on:click={() => handlePaint(83)} on:mouseenter={() => mousedown === true && handlePaint(83)} on:keydown={(e) => e.key === 'Enter' && handlePaint(83)}/>
        <rect x="696" y="180" width="35" height="35" rx="4" fill={keyColours[84]} role="button" tabindex=0 on:click={() => handlePaint(84)} on:mouseenter={() => mousedown === true && handlePaint(84)} on:keydown={(e) => e.key === 'Enter' && handlePaint(84)}/>
        <rect x="696" y="223" width="35" height="35" rx="4" fill={keyColours[85]} role="button" tabindex=0 on:click={() => handlePaint(85)} on:mouseenter={() => mousedown === true && handlePaint(85)} on:keydown={(e) => e.key === 'Enter' && handlePaint(85)}/>
        <rect x="739" width="35" height="35" rx="4" fill={keyColours[86]} role="button" tabindex=0 on:click={() => handlePaint(86)} on:mouseenter={() => mousedown === true && handlePaint(86)} on:keydown={(e) => e.key === 'Enter' && handlePaint(86)}/>
        <rect x="739" y="51" width="35" height="35" rx="4" fill={keyColours[87]} role="button" tabindex=0 on:click={() => handlePaint(87)} on:mouseenter={() => mousedown === true && handlePaint(87)} on:keydown={(e) => e.key === 'Enter' && handlePaint(87)}/>
        <rect x="739" y="94" width="35" height="35" rx="4" fill={keyColours[88]} role="button" tabindex=0 on:click={() => handlePaint(88)} on:mouseenter={() => mousedown === true && handlePaint(88)} on:keydown={(e) => e.key === 'Enter' && handlePaint(88)}/>
        <rect x="739" y="223" width="35" height="35" rx="4" fill={keyColours[89]} role="button" tabindex=0 on:click={() => handlePaint(89)} on:mouseenter={() => mousedown === true && handlePaint(89)} on:keydown={(e) => e.key === 'Enter' && handlePaint(89)}/>
        <rect x="790" y="51" width="35" height="35" rx="4" fill={keyColours[90]} role="button" tabindex=0 on:click={() => handlePaint(90)} on:mouseenter={() => mousedown === true && handlePaint(90)} on:keydown={(e) => e.key === 'Enter' && handlePaint(90)}/>
        <rect x="790" y="94" width="35" height="35" rx="4" fill={keyColours[91]} role="button" tabindex=0 on:click={() => handlePaint(91)} on:mouseenter={() => mousedown === true && handlePaint(91)} on:keydown={(e) => e.key === 'Enter' && handlePaint(91)}/>
        <rect x="790" y="137" width="35" height="35" rx="4" fill={keyColours[92]} role="button" tabindex=0 on:click={() => handlePaint(92)} on:mouseenter={() => mousedown === true && handlePaint(92)} on:keydown={(e) => e.key === 'Enter' && handlePaint(92)}/>
        <rect x="790" y="180" width="35" height="35" rx="4" fill={keyColours[93]} role="button" tabindex=0 on:click={() => handlePaint(93)} on:mouseenter={() => mousedown === true && handlePaint(93)} on:keydown={(e) => e.key === 'Enter' && handlePaint(93)}/>
        <rect x="790" y="223" width="78" height="35" rx="4" fill={keyColours[94]} role="button" tabindex=0 on:click={() => handlePaint(94)} on:mouseenter={() => mousedown === true && handlePaint(94)} on:keydown={(e) => e.key === 'Enter' && handlePaint(94)}/>
        <rect x="833" y="51" width="35" height="35" rx="4" fill={keyColours[95]} role="button" tabindex=0 on:click={() => handlePaint(95)} on:mouseenter={() => mousedown === true && handlePaint(95)} on:keydown={(e) => e.key === 'Enter' && handlePaint(95)}/>
        <rect x="833" y="94" width="35" height="35" rx="4" fill={keyColours[96]} role="button" tabindex=0 on:click={() => handlePaint(96)} on:mouseenter={() => mousedown === true && handlePaint(96)} on:keydown={(e) => e.key === 'Enter' && handlePaint(96)}/>
        <rect x="833" y="137" width="35" height="35" rx="4" fill={keyColours[97]} role="button" tabindex=0 on:click={() => handlePaint(97)} on:mouseenter={() => mousedown === true && handlePaint(97)} on:keydown={(e) => e.key === 'Enter' && handlePaint(97)}/>
        <rect x="833" y="180" width="35" height="35" rx="4" fill={keyColours[98]} role="button" tabindex=0 on:click={() => handlePaint(98)} on:mouseenter={() => mousedown === true && handlePaint(98)} on:keydown={(e) => e.key === 'Enter' && handlePaint(98)}/>
        <rect x="876" y="51" width="35" height="35" rx="4" fill={keyColours[99]} role="button" tabindex=0 on:click={() => handlePaint(99)} on:mouseenter={() => mousedown === true && handlePaint(99)} on:keydown={(e) => e.key === 'Enter' && handlePaint(99)}/>
        <rect x="876" y="94" width="35" height="35" rx="4" fill={keyColours[100]} role="button" tabindex=0 on:click={() => handlePaint(100)} on:mouseenter={() => mousedown === true && handlePaint(100)} on:keydown={(e) => e.key === 'Enter' && handlePaint(100)}/>
        <rect x="876" y="137" width="35" height="35" rx="4" fill={keyColours[101]} role="button" tabindex=0 on:click={() => handlePaint(101)} on:mouseenter={() => mousedown === true && handlePaint(101)} on:keydown={(e) => e.key === 'Enter' && handlePaint(101)}/>
        <rect x="876" y="180" width="35" height="35" rx="4" fill={keyColours[102]} role="button" tabindex=0 on:click={() => handlePaint(102)} on:mouseenter={() => mousedown === true && handlePaint(102)} on:keydown={(e) => e.key === 'Enter' && handlePaint(102)}/>
        <rect x="876" y="223" width="35" height="35" rx="4" fill={keyColours[103]} role="button" tabindex=0 on:click={() => handlePaint(103)} on:mouseenter={() => mousedown === true && handlePaint(103)} on:keydown={(e) => e.key === 'Enter' && handlePaint(103)}/>
        <rect x="919" y="51" width="35" height="35" rx="4" fill={keyColours[104]} role="button" tabindex=0 on:click={() => handlePaint(104)} on:mouseenter={() => mousedown === true && handlePaint(104)} on:keydown={(e) => e.key === 'Enter' && handlePaint(104)}/>
        <rect x="919" y="94" width="35" height="78" rx="4" fill={keyColours[105]} role="button" tabindex=0 on:click={() => handlePaint(105)} on:mouseenter={() => mousedown === true && handlePaint(105)} on:keydown={(e) => e.key === 'Enter' && handlePaint(105)}/>
        <rect x="919" y="180" width="35" height="78" rx="4" fill={keyColours[106]} role="button" tabindex=0 on:click={() => handlePaint(106)} on:mouseenter={() => mousedown === true && handlePaint(106)} on:keydown={(e) => e.key === 'Enter' && handlePaint(106)}/>
    </svg>
        
    <input type="color" value="#E5E7EB" width="78" height="35" class="rounded-lg my-10" />
</div>

<style>
    rect:hover {
        fill: #E5E7EB;
    }
</style>