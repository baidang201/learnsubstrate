import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kittyCnt, setKittyCnt] = useState(0)
  const [kittyDNAs, setKittyDNAs] = useState([])
  const [kittyOwners, setKittyOwners] = useState([])
  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const fetchKittyCnt = () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    api.query.kittiesModule.kittiesCount(amount => {
      // 获取猫咪总数/ID
      const kittyCount = amount.toJSON()
      setKittyCnt(kittyCount)
    })
  }

  const fetchKittiesOwner = () => {
    // 获取猫咪的主人
    api.query.kittiesModule.owner.multi([...Array(kittyCnt).keys()], (data) => {
      const tempData = []
      data.map(row => {
        if (row.isNone) {
          tempData.push('猫不存在')
        } else {
          // Option 转字符串
          // hash.toJSON() 或 hash.toHuman()
          // value.toJSON() 或 value.toHuman()
          const kittyOwner = row.value.toHuman()

          tempData.push(kittyOwner)
        }
      })
      setKittyOwners(tempData)
    })
    // debugger;
  }

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态

    api.query.kittiesModule.kitties.multi([...Array(kittyCnt).keys()], (data) => {
      const tempData = []
      data.map(row => {
        if (row.isNone) {
          tempData.push('no kitty')
        } else {
          const kittyDna = row.value.toU8a()
          // console.log('dna == ' + kittyDna)
          tempData.push(kittyDna)
        }
      })
      setKittyDNAs(tempData)
    })
  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    const kittiesAllInfo = []

    for (let idx = 0; idx < kittyCnt; idx++) {
      kittiesAllInfo.push({
        id: idx,
        dna: kittyDNAs[idx],
        owner: kittyOwners[idx]
      })
    }

    setKitties(kittiesAllInfo)
  }

  useEffect(fetchKittyCnt, [api, keyring])
  useEffect(fetchKitties, [api, kittyDNAs, kittyOwners])
  useEffect(fetchKittiesOwner, [api, kittyDNAs, kittyOwners])
  useEffect(populateKitties, [kittyDNAs, kittyOwners])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
