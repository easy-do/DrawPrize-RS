import React, { useContext, useRef } from 'react';
import {
  Form,
  FormInstance,
  Input,
  Modal,
  Notification,
  Select,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { addPrizePool } from '@/api/prizePool';
import FormItem from '@arco-design/web-react/es/Form/form-item';

function AddPage(props: { visible; setVisible; callback: () => void }) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);

  const [loading, setLoading] = React.useState(false);

  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      values.status = values.status == 'true';
      values.share_pool = values.share_pool == 'true';
      addPrizePool(values)
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
          label={t['searchTable.columns.pool_name']}
          field={'pool_name'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        {/* <FormItem
          required
          label={t['searchTable.columns.pool_type']}
          field={'pool_type'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem> */}
        <FormItem
          required
          initialValue={'false'}
          label={t['searchTable.columns.share_pool']}
          field={'share_pool'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchTable.columns.yes'],
                value: 'true',
              },
              {
                label: t['searchTable.columns.no'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
        {/* <FormItem
          required
          label={t['searchTable.columns.strategy']}
          field={'strategy'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem> */}
        <FormItem
          required
          label={t['searchTable.columns.pool_desc']}
          field={'pool_desc'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          initialValue={'true'}
          label={t['searchTable.columns.status']}
          field={'status'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchForm.enable'],
                value: 'true',
              },
              {
                label: t['searchForm.disable'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default AddPage;
