import React, { useContext, useEffect, useRef } from 'react';
import {
  Form,
  FormInstance,
  Modal,
  Notification,
  Select,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { addLivePrizePoolItem } from '@/api/livePrizePoolItem';
import { getPrizePoolItemBtPoolId } from '@/api/prizePoolItem';

function AddPage(props: {
  visible;
  setVisible;
  livePrizePoolId;
  callback: () => void;
}) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);

  const [loading, setLoading] = React.useState(false);

  const [itemSelect, setItemSelect] = React.useState([]);


  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      addLivePrizePoolItem(props.livePrizePoolId, values.item_id)
        .then((res) => {
          const { success, message } = res.data;
          if (success) {
            Notification.success({ content: message, duration: 1000 });
            props.setVisible(false);
            props.callback();
          }
        })
        .finally(() => {
          setLoading(false);
        });
    });
  };

  useEffect(() => {
    setLoading(true);
    getPrizePoolItemBtPoolId(props.livePrizePoolId).then((res) => {
      const { success, data } = res.data;
      if (success) {
        setItemSelect(data.map((item) => {
          return {
            label: item.prize_name,
            value: item.id,
          };
        }));
      }
    }).finally(() => {
      setLoading(false);
    });

  }, [props.visible]);

  return (
    <Modal
      title={t['searchTable.operations.add']}
      style={{ width: '35%' }}
      visible={props.visible}
      onOk={() => {
        handleSubmit();
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      autoFocus={false}
      focusLock={true}
      confirmLoading={loading}
    >
      <Form
        ref={formRef}
        style={{ width: '95%', marginTop: '6px' }}
        labelCol={{ span: lang === 'en-US' ? 7 : 6 }}
        wrapperCol={{ span: lang === 'en-US' ? 17 : 18 }}
      >
        <FormItem
          required
          label={t['searchTable.columns.item_id']}
          field={'item_id'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={itemSelect}
            allowClear
          />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default AddPage;
