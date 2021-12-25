<script lang="ts">
import { defineComponent } from 'vue';
import Configs from '../ts/Configs';

import SelectionList from './bases/SelectionList.vue';

export default defineComponent({
    mixins: [SelectionList]
});
</script>
