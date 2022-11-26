<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Register Successful</name>
   <tag></tag>
   <elementGuidId>a5b74428-1680-4ab4-9a57-372cd0194df4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;${email}\&quot;: \&quot;${emailvalue}\&quot;,\n    \&quot;${password}\&quot;: \&quot;${passwordvalue}\&quot;\n}&quot;,
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
      <webElementGuid>27b4b20d-61c7-4546-8707-e8eee8d06923</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseURL}${POSTRegisterSuccessful}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseURL</defaultValue>
      <description></description>
      <id>d9befee2-abe8-4d86-b02c-5e73f5800c47</id>
      <masked>false</masked>
      <name>BaseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.POSTRegisterSuccessful</defaultValue>
      <description></description>
      <id>d7136d60-b7b0-41f2-9661-b2d96b6c03f5</id>
      <masked>false</masked>
      <name>POSTRegisterSuccessful</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.email</defaultValue>
      <description></description>
      <id>412abe6b-7129-4733-8767-54addc0f8baa</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.emailvalue</defaultValue>
      <description></description>
      <id>a8af798d-fefb-4ee9-bf16-b10a44524709</id>
      <masked>false</masked>
      <name>emailvalue</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>07221e73-5808-4adc-84ff-4da5930139a2</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.passwordvalue</defaultValue>
      <description></description>
      <id>6eaedd2d-0706-41d5-9409-f0aac8c7c019</id>
      <masked>false</masked>
      <name>passwordvalue</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'id', 2)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
