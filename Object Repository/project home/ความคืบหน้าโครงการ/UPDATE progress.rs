<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE progress</name>
   <tag></tag>
   <elementGuidId>f8641b2e-ac80-495e-a77f-e723a30f2fe4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id_progress}\&quot;,\n  \&quot;name\&quot;: {\n    \&quot;th\&quot;: \&quot;ความคืบหน้าครั้งที่ 1_edit\&quot;,\n    \&quot;en\&quot;: \&quot;Progress No.1_edit\&quot;\n  },\n  \&quot;items\&quot;: [\n    {\n      \&quot;id\&quot;: \&quot;${id_itemprogress}\&quot;,\n      \&quot;percentage\&quot;: 3,\n      \&quot;selected\&quot;: true,\n      \&quot;name\&quot;: {\n        \&quot;th\&quot;: \&quot;วางโครงสร้าง_edit\&quot;,\n        \&quot;en\&quot;: \&quot;วางโครงสร้าง_edit\&quot;\n      },\n      \&quot;description\&quot;: {\n        \&quot;th\&quot;: \&quot;วางโครงสร้าง_edit\&quot;,\n        \&quot;en\&quot;: \&quot;วางโครงสร้าง_edit\&quot;\n      }\n    }\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>348ff76d-2d1a-44d1-8088-53b659c165d4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>80cdc515-f8e4-4f33-875d-56622a1412f1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/${id_project}/progress</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>d186761c-c7db-467d-8d5d-9e3bf349a969</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_project</defaultValue>
      <description></description>
      <id>198d6a2c-3f24-478a-874c-8c6c92596c0e</id>
      <masked>false</masked>
      <name>id_project</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_progress</defaultValue>
      <description></description>
      <id>361c7d9b-99ce-4e6c-b046-18de5ba750b7</id>
      <masked>false</masked>
      <name>id_progress</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_itemprogress</defaultValue>
      <description></description>
      <id>12a52c9f-be24-4583-8b92-96cfccc8261f</id>
      <masked>false</masked>
      <name>id_itemprogress</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
